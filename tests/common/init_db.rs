use aruna_server::database::connection::Database;

#[allow(dead_code)]
pub async fn init_db() -> Database {
    let database_host = "localhost";
    let database_name = "test";
    let database_port = 5433;
    let database_user = "yugabyte";
    Database::new(database_host, database_port, database_name, database_user).unwrap()
    //db.initialize_db().await.unwrap();
}
