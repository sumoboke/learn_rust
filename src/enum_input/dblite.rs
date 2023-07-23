use sqlx::{SqliteConnection, Connection};

#[derive(Debug)]
struct Vehicle {
    id: u32,
    vehicle_num: String,
    driver_name: String,
    from_location: String,
    to_location: String,
    time_drive: String,
    is_done: bool
}

enum DataAction {
    Create(Vehicle),
    Read,
    Update(u32, Vehicle),
    Delete(u32),
    ReadAll
}

struct Database {
    conn : SqliteConnection
}

impl Database {
   async fn new(filename : &str) -> sqlx::Result<Self> {
    let conn = SqliteConnection::connect(filename).await?;
        Ok(Database{conn})

    }
}
