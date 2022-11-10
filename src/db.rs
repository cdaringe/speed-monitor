use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(Debug)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn create(dbconnstr: &str) -> Result<Db, std::string::String> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(dbconnstr)
            .await
            .map_err(|err| format!("{}, {}", err.to_string(), dbconnstr))?;
        Ok(Db { pool })
    }

    pub async fn migrate(&self) -> () {
        let mut pool = self.pool.acquire().await.unwrap();
        sqlx::migrate!("db/migrations")
            .run(&mut pool)
            .await
            .expect("migrations");
    }
}
