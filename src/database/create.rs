extern crate directories;
use directories::BaseDirs;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn create_project_resources() -> SqlitePool {
    create_directories().await;
    create_migrations().await;
    create_db().await
}

async fn create_directories() {
    let root = get_root_path_buf();
    let _ = fs::create_dir_all(root.join("flashcards/sql/migrations"));
}

async fn create_migrations() {
    let migrations_path = get_migrations_path_buf();
    let migrations_0 = File::create(migrations_path.join("0_migration.sql"));
    let migration_0_content = b"CREATE TABLE IF NOT EXISTS cards (
                                            id                  INTEGER PRIMARY KEY NOT NULL,
                                            question            VARCHAR(250)        NOT NULL,
                                            answer              VARCHAR(250)        NOT NULL,
                                            shown               INTEGER             NOT NULL
                                            );";

    let migrations = [(migrations_0, migration_0_content)];
    for m in migrations.into_iter() {
        let mut file = m.0.await.unwrap();
        let _ = file.write(m.1);
        let _ = file.flush();
    }
}

async fn create_db() -> SqlitePool {
    let db_url = get_db_path_buf().to_str().unwrap().to_owned();

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        let _ = Sqlite::create_database(&db_url).await;
    }

    let db = SqlitePool::connect(&db_url).await.unwrap();
    let migrations_path = get_migrations_path_buf();

    let _ = sqlx::migrate::Migrator::new(migrations_path)
        .await
        .unwrap()
        .run(&db)
        .await;

    db
}

fn get_root_path_buf() -> PathBuf {
    BaseDirs::new().unwrap().data_dir().to_path_buf()
}

fn get_migrations_path_buf() -> PathBuf {
    get_root_path_buf().join("flashcards/sql/migrations")
}

fn get_db_path_buf() -> PathBuf {
    let root = get_root_path_buf();
    let mut sql_path = "sqlite://".to_owned();
    sql_path.push_str(root.to_str().unwrap());
    sql_path.push_str("/flashcards/sql/flashcards.db");
    let mut sql_path_buf = PathBuf::new();
    sql_path_buf.push(sql_path);
    sql_path_buf
}
