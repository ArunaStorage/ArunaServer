use crate::database::crud::{CrudDb, PrimaryKey};
use anyhow::Result;
use postgres_from_row::FromRow;
use tokio_postgres::Client;

#[derive(FromRow, Debug)]
pub struct PubKeys {
    pub id: i32,
    pub pubkey: String,
}

#[async_trait::async_trait]
impl CrudDb for PubKeys {
    async fn create(&self, client: &Client) -> Result<()> {
        let query = "INSERT INTO pub_keys (id, pubkey) VALUES ($1, $2);";
        let prepared = client.prepare(query).await?;
        client.query(&prepared, &[&self.id, &self.pubkey]).await?;
        Ok(())
    }
    async fn get(id: impl PrimaryKey, client: &Client) -> Result<Option<Self>> {
        // This needs to be dynamic for (origin/target)/(did/pid)
        let query = "SELECT * FROM pub_keys WHERE id = $1";
        let prepared = client.prepare(query).await?;
        Ok(client
            .query_opt(&prepared, &[&id])
            .await?
            .map(|e| PubKeys::from_row(&e)))
    }
    async fn all(client: &Client) -> Result<Vec<Self>> {
        let query = "SELECT * FROM pub_keys";
        let prepared = client.prepare(query).await?;
        let rows = client.query(&prepared, &[]).await?;
        Ok(rows.iter().map(PubKeys::from_row).collect::<Vec<_>>())
    }

    async fn delete(&self, id: impl PrimaryKey, client: &Client) -> Result<()> {
        let query = "DELETE FROM pub_keys WHERE id = $1";
        let prepared = client.prepare(query).await?;
        client.execute(&prepared, &[&id]).await?;
        Ok(())
    }
}
