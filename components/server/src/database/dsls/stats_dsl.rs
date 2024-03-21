use std::{collections::BTreeSet, hash::Hash, sync::Arc, time::Duration};

use anyhow::{bail, Result};
use async_channel::Receiver;
use chrono::{NaiveDateTime, Utc};
use diesel_ulid::DieselUlid;
use itertools::Itertools;
use log::error;
use postgres_from_row::FromRow;
use tokio_postgres::Client;

use crate::{
    caching::cache::Cache,
    database::connection::Database,
    notification::natsio_handler::{NatsIoHandler, ServerEvents},
    search::meilisearch_client::{MeilisearchClient, ObjectDocument},
    utils::search_utils,
};

#[derive(Copy, Clone, Debug, FromRow, Hash)]
pub struct ObjectStats {
    pub count: i64,
    pub size: i64,
    pub last_refresh: NaiveDateTime,
}

pub async fn start_refresh_loop(
    database: Arc<Database>,
    cache: Arc<Cache>,
    search_client: Arc<MeilisearchClient>,
    natsio_handler: Arc<NatsIoHandler>,
    refresh_receiver: Receiver<i64>,
    refresh_interval: i64,
) {
    // Start loop
    tokio::spawn(async move {
        let mut last_object_stats = BTreeSet::new();
        loop {
            // Try to get database connection
            let client = match database.get_client().await {
                Ok(client) => client,
                Err(err) => {
                    error!("Failed to get database client for MV refresh: {}", err);
                    tokio::time::sleep(Duration::from_secs(15)).await; // Wait 15s and try again
                    continue;
                }
            };

            // Save current timestamp to check if refresh is necessary
            let mut current_timestamp = Utc::now().timestamp_millis();

            // Collect latest timestamp of all other started refreshs
            let mut latest_refresh = None;
            while let Ok(refresh_timestamp) = refresh_receiver.try_recv() {
                if let Some(latest_timestamp) = latest_refresh {
                    if refresh_timestamp > latest_timestamp {
                        latest_refresh = Some(refresh_timestamp)
                    }
                } else {
                    latest_refresh = Some(refresh_timestamp)
                }
            }

            // Evaluate if refresh is necessary
            let start_refresh = if let Some(refresh_timestamp) = latest_refresh {
                let start_refresh = (current_timestamp - refresh_timestamp) > refresh_interval;
                if !start_refresh {
                    current_timestamp = refresh_timestamp
                }
                start_refresh
            } else {
                true
            };

            // Start MV refresh if conditions are met
            if start_refresh {
                match refresh_stats_view(&client).await {
                    Ok(_) => {
                        // Send notification that MV refresh has been started
                        if let Err(err) = natsio_handler
                            .register_server_event(ServerEvents::MVREFRESH(current_timestamp))
                            .await
                        {
                            error!("Failed to send MV refresh notification: {}", err)
                        }
                    }
                    Err(err) => {
                        error!("Start MV refresh failed: {}", err);
                        // Sleep for refresh interval and try again
                        tokio::time::sleep(Duration::from_millis(
                            refresh_interval.try_into().unwrap_or(30000),
                        ))
                        .await;
                        continue;
                    }
                }
            }

            // Wait in every case for refresh to finish for cache update
            while let Ok(last_refresh) = get_last_refresh(&client).await {
                if last_refresh.and_utc().timestamp_millis() >= current_timestamp {
                    // Fetch all ObjectStats, create diff with last loop iteration and update only changed
                    let object_stats =
                        BTreeSet::from_iter(match ObjectStats::get_all_stats(&client).await {
                            Ok(stats) => stats,
                            Err(err) => {
                                error!("Failed to fetch all stats from database: {}", err);
                                break;
                            }
                        });

                    let diff = object_stats
                        .difference(&last_object_stats)
                        .cloned()
                        .collect_vec();

                    // Save current stats for next iteration
                    last_object_stats = BTreeSet::from_iter(object_stats);

                    // Update changed stats in cache only if stats are available and anything has changed
                    if !diff.is_empty() {
                        if let Err(err) = cache.upsert_object_stats(diff.clone()).await {
                            error!("Object stats cache update failed: {}", err)
                        } else {
                            // Update changes in search index
                            let ods: Result<Vec<ObjectDocument>> = diff
                                .iter()
                                .map(|os| -> Result<ObjectDocument> {
                                    cache.get_object_document(&os.origin_pid).ok_or_else(|| {
                                        anyhow::anyhow!(
                                            "Could not find object {} in cache",
                                            os.origin_pid
                                        )
                                    })
                                })
                                .collect();

                            match ods {
                                Ok(index_updates) => {
                                    search_utils::update_search_index(
                                        &search_client,
                                        &cache,
                                        index_updates,
                                    )
                                    .await;
                                }
                                Err(err) => error!(
                                    "Failed to fetch objects for search index update: {}",
                                    err
                                ),
                            }
                        };
                    }
                    break;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await // Wait 1 sec and try again
                }
            }

            // Sleep for refresh interval
            tokio::time::sleep(Duration::from_millis(
                refresh_interval.try_into().unwrap_or(30000),
            ))
            .await;
        }

        //Ok::<(), anyhow::Error>(())
    });
}
