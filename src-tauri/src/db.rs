use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::fs;
use std::path::PathBuf;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Ensure the app directory exists
        fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("data.db");

        println!("-----------------------------------------------");
        println!("Initializing database at: {:?}", db_path);
        println!("-----------------------------------------------");


        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        // Create and initialize the database pool
        let pool = SqlitePool::connect_with(connection_options).await?;

        // Run migrations regardless of whether the database is new
        // SQLx will track which migrations have been run
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

// State management for Tauri
pub struct DatabaseState(pub Pool<Sqlite>);