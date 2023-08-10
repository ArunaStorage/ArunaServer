use crate::database::crud::{CrudDb, PrimaryKey};
use anyhow::Result;
use diesel_ulid::DieselUlid;
use postgres_from_row::FromRow;
use tokio_postgres::Client;

#[derive(FromRow, Debug)]
pub struct PubKey {
    pub id: i16,
    pub proxy: Option<DieselUlid>,
    pub pubkey: String,
}

#[async_trait::async_trait]
impl CrudDb for PubKey {
    async fn create(&self, client: &Client) -> Result<()> {
        let query =
            "INSERT INTO pub_keys (id, proxy, pubkey) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING;";
        let prepared = client.prepare(query).await?;
        client
            .query(&prepared, &[&self.id, &self.proxy, &self.pubkey])
            .await?;
        Ok(())
    }
    async fn get(id: impl PrimaryKey, client: &Client) -> Result<Option<Self>> {
        // This needs to be dynamic for (origin/target)/(did/pid)
        let query = "SELECT * FROM pub_keys WHERE id = $1";
        let prepared = client.prepare(query).await?;
        Ok(client
            .query_opt(&prepared, &[&id])
            .await?
            .map(|e| PubKey::from_row(&e)))
    }
    async fn all(client: &Client) -> Result<Vec<Self>> {
        let query = "SELECT * FROM pub_keys";
        let prepared = client.prepare(query).await?;
        let rows = client.query(&prepared, &[]).await?;
        Ok(rows.iter().map(PubKey::from_row).collect::<Vec<_>>())
    }

    async fn delete(&self, client: &Client) -> Result<()> {
        let query = "DELETE FROM pub_keys WHERE id = $1";
        let prepared = client.prepare(query).await?;
        client.execute(&prepared, &[&self.id]).await?;
        Ok(())
    }
}

impl PubKey {
    /// Queries the pub_keys table for a specific pubkey.
    /// As the pubkeys are unique on the database level it returns an Option which includes the
    /// pubkey if it exists.
    pub async fn get_by_key(pubkey: &str, client: &Client) -> Result<Option<PubKey>> {
        let query = "SELECT * FROM pub_keys WHERE pubkey = $1";
        let prepared = client.prepare(query).await?;
        Ok(client
            .query_opt(&prepared, &[&pubkey])
            .await?
            .map(|e| PubKey::from_row(&e)))
    }
}
