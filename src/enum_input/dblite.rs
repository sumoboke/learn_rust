mod submit_db;
mod update_db;

use serde::{Deserialize, Serialize};
use sqlx::{Connection, FromRow, SqliteConnection};
use submit_db::write_data;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Truck {
    #[sqlx(default)]
    id: Option<i64>,
    #[sqlx(default)]
    time_submit: Option<i64>,
    vehicle_num: String,
    pub driver_name: String,
    from_location: String,
    to_location: String,
    time_drive: i64,
    #[sqlx(default)]
    is_done: Option<bool>,
}

enum DataAction {
    Create(Truck),
    Read,
    Update(u32, Truck),
    Delete(u32),
    ReadAll,
}

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    pub async fn new(filename: &str) -> sqlx::Result<Self> {
        let conn = SqliteConnection::connect(filename).await?;
        Ok(Database { conn })
    }

    async fn create(path: &str, truck: &Truck) -> sqlx::Result<()> {
        let mut db = Self::new(path).await.unwrap();

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
        .execute(&mut db.conn)
        .await?;

        Ok(())
    }

    pub async fn get_detail_by_num(&mut self, truck_num: &str) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            SELECT * FROM trucks
            where vehicle_num = ?
            "#,
            truck_num
        )
        .fetch_all(&mut self.conn)
        .await?;

        Ok(())
    }

    pub async fn submit_data(path: &str) -> sqlx::Result<()> {
        let data = write_data();
        Self::create(path, &data).await?;
        Ok(())
    }

    pub async fn read_all(path: &str) -> sqlx::Result<Vec<Truck>> {
        let mut db = Self::new(path).await.unwrap();
        println!("All Trucks");

        let trucks = sqlx::query_as!(Truck, r#"SELECT * FROM trucks"#)
            .fetch_all(&mut db.conn)
            .await?;

        Ok(trucks)
    }
}
