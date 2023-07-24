use sqlx::{query_as, sqlite::SqliteQueryAs, Connect, Connection, SqliteConnection};

#[derive(sqlx::FromRow)]
struct Vehicle {
    id: i32,
    vehicle_num: String,
    driver: String,
    from_location: String,
    to_location: String,
    time_drive: String,
}

enum DatabaseAction {
    Read,
    Write(Vehicle),
    ReadAll,
    Update(i32, Vehicle),
}

struct Database {
    conn: SqliteConnection,
}

impl Database {
    // Function to create a new database and connect to it
    async fn new(filename: &str) -> sqlx::Result<Self> {
        let conn = SqliteConnection::connect(filename).await?;
        Ok(Database { conn })
    }

    // Function to perform database actions
    async fn perform_action(&self, action: DatabaseAction) -> sqlx::Result<()> {
        match action {
            DatabaseAction::Read => self.read().await?,
            DatabaseAction::Write(vehicle) => self.write(vehicle).await?,
            DatabaseAction::ReadAll => self.read_all().await?,
            DatabaseAction::Update(id, vehicle) => self.update(id, vehicle).await?,
        }
        Ok(())
    }

    // Function to create the database schema and insert data into the table
    async fn write(&self, vehicle: Vehicle) -> sqlx::Result<()> {
        // Insert data into the table
        sqlx::query("INSERT INTO vehicles (vehicle_num, driver, from_location, to_location, time_drive) VALUES (?, ?, ?, ?, ?)")
            .bind(&vehicle.vehicle_num)
            .bind(&vehicle.driver)
            .bind(&vehicle.from_location)
            .bind(&vehicle.to_location)
            .bind(&vehicle.time_drive)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    // Function to read data from the database and print it
    async fn read(&self) -> sqlx::Result<()> {
        println!("Vehicles:");
        let vehicles: Vec<Vehicle> = query_as!(
            Vehicle,
            "SELECT id, vehicle_num, driver, from_location, to_location, time_drive FROM vehicles"
        )
        .fetch_all(&self.conn)
        .await?;

        for vehicle in vehicles {
            println!(
                "ID: {}, Vehicle Num: {}, Driver: {}, From: {}, To: {}, Time Drive: {}",
                vehicle.id,
                vehicle.vehicle_num,
                vehicle.driver,
                vehicle.from_location,
                vehicle.to_location,
                vehicle.time_drive
            );
        }

        Ok(())
    }

    // Function to read all data from the database and print it
    async fn read_all(&self) -> sqlx::Result<()> {
        println!("All Vehicles:");
        println!("connection {}", &self.conn);
        let vehicles: Vec<Vehicle> = query_as!(Vehicle, "SELECT * FROM vehicles")
            .fetch_all(&self.conn)
            .await?;

        for vehicle in vehicles {
            println!(
                "ID: {}, Vehicle Num: {}, Driver: {}, From: {}, To: {}, Time Drive: {}",
                vehicle.id,
                vehicle.vehicle_num,
                vehicle.driver,
                vehicle.from_location,
                vehicle.to_location,
                vehicle.time_drive
            );
        }

        Ok(())
    }

    // Function to update data in the database
    async fn update(&self, id: i32, vehicle: Vehicle) -> sqlx::Result<()> {
        sqlx::query("UPDATE vehicles SET vehicle_num = ?, driver = ?, from_location = ?, to_location = ?, time_drive = ? WHERE id = ?")
            .bind(&vehicle.vehicle_num)
            .bind(&vehicle.driver)
            .bind(&vehicle.from_location)
            .bind(&vehicle.to_location)
            .bind(&vehicle.time_drive)
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    // Function to get the total number of data rows in the database
    async fn get_total_rows(&self) -> sqlx::Result<usize> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vehicles")
            .fetch_one(&self.conn)
            .await?;
        Ok(count.0 as usize)
    }
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    // Create a new database or connect to an existing one named "data.db"
    let database = Database::new("data.db").await?;

    // Call the methods to perform different actions
    database
        .perform_action(DatabaseAction::Write(Vehicle {
            id: 0,
            vehicle_num: "ABC123".to_string(),
            driver: "John Doe".to_string(),
            from_location: "City A".to_string(),
            to_location: "City B".to_string(),
            time_drive: "9:00 AM".to_string(),
        }))
        .await?;

    database
        .perform_action(DatabaseAction::Write(Vehicle {
            id: 0,
            vehicle_num: "XYZ456".to_string(),
            driver: "Jane Smith".to_string(),
            from_location: "City C".to_string(),
            to_location: "City D".to_string(),
            time_drive: "2:30 PM".to_string(),
        }))
        .await?;

    database.perform_action(DatabaseAction::Read).await?;
    database.perform_action(DatabaseAction::ReadAll).await?;

    database
        .perform_action(DatabaseAction::Update(
            1,
            Vehicle {
                id: 0,
                vehicle_num: "DEF789".to_string(),
                driver: "Updated Driver".to_string(),
                from_location: "City E".to_string(),
                to_location: "City F".to_string(),
                time_drive: "6:00 PM".to_string(),
            },
        ))
        .await?;

    database.perform_action(DatabaseAction::ReadAll).await?;

    // Get the total number of data rows and print it
    let total_rows = database.get_total_rows().await?;
    println!("Total number of data rows: {}", total_rows);

    Ok(())
}
