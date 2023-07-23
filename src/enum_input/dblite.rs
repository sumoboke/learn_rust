use sqlx::{Connection, SqliteConnection};
#[derive(Debug)]
struct Truck {
    id: u32,
    time_submit: u64,
    vehicle_num: String,
    driver_name: String,
    from_location: String,
    to_location: String,
    time_drive: u64,
    is_done: bool,
}

enum DataAction {
    Create(Truck),
    Read,
    Update(u32, Truck),
    Delete(u32),
    ReadAll,
}

struct Database {
    conn: SqliteConnection,
}

impl Database {
    async fn new(filename: &str) -> sqlx::Result<Self> {
        let conn = SqliteConnection::connect(filename).await?;
        Ok(Database { conn })
    }

    async fn create(&self, truck: Truck) {}
}
