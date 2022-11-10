use dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tokio;

fn main() {
    println!("sup, build starting");
    println!("cargo:rerun-if-changed=db/migrations");
    println!("cargo:rerun-if-changed=.env");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            dotenv::dotenv().ok().expect(".env ok");
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap();
            let mut conn = pool.acquire().await.unwrap();
            sqlx::migrate!("db/migrations")
                .run(&mut conn)
                .await
                .unwrap();
        });
    println!("sup, build ending");
}
