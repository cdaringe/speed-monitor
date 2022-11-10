use dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tokio;

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=.env");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // db migrations!
            println!("starting dotenv");
            dotenv::dotenv().ok().expect(".env ok");
            println!("starting sqlite");
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap();
            println!("getting pool");
            let mut conn = pool.acquire().await.unwrap();
            println!("migrating");
            sqlx::migrate!().run(&mut conn).await.expect("jamon");
            println!("migrated");
        });
}
