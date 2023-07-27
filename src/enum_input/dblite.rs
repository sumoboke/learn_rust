use sqlx::{Connection, FromRow, SqliteConnection};
#[derive(Debug, FromRow)]
struct Truck {
    id: u32,
    time_submit: i64,
    vehicle_num: String,
    driver_name: String,
    from_location: String,
    to_location: String,
    time_drive: i64,
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

    async fn create(&mut self, truck: &Truck) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO trucks
      (vehicle_num, driver_name, from_location, to_location, time_drive)
       VALUES ($1, $2, $3, $4, $5 )
       "#,
            truck.vehicle_num,
            truck.driver_name,
            truck.from_location,
            truck.to_location,
            truck.time_drive,
        )
        .execute(&mut self.conn)
        .await?;

        Ok(())
    }

async fn get_detail_by_num(&mut self, &str)
}
