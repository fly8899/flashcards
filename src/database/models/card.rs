use sqlx::{sqlite::SqliteQueryResult, Error, SqlitePool};

use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Card {
    pub id: i32,
    pub question: String,
    pub answer: String,
}

pub async fn add_card(
    pool: &SqlitePool,
    question: &String,
    answer: &String,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO cards (question, answer, shown) VALUES (?,?,?)")
        .bind(question)
        .bind(answer)
        .bind(false)
        .execute(pool)
        .await
}

pub async fn update_card(
    pool: &SqlitePool,
    id: &i32,
    question: &String,
    answer: &String,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("UPDATE cards SET question = ?, answer = ? WHERE id = ?")
        .bind(question)
        .bind(answer)
        .bind(id)
        .execute(pool)
        .await
}

pub async fn mark_card_shown(pool: &SqlitePool, id: &i32) -> Result<SqliteQueryResult, Error> {
    sqlx::query("UPDATE cards SET shown = ? WHERE id = ?")
        .bind(true)
        .bind(id)
        .execute(pool)
        .await
}

pub async fn mark_cards_not_shown(pool: &SqlitePool) -> Result<SqliteQueryResult, Error> {
    sqlx::query("UPDATE cards SET shown = ?")
        .bind(false)
        .execute(pool)
        .await
}

pub async fn remove_card(pool: &SqlitePool, id: &i32) -> Result<SqliteQueryResult, Error> {
    sqlx::query("DELETE FROM cards WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
}

pub async fn fetch_random_card_marked_not_shown(
    pool: &SqlitePool,
) -> Result<Option<Card>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM cards WHERE shown = 0 ORDER BY RANDOM() LIMIT 1")
        .fetch_optional(pool)
        .await
}

pub async fn fetch_all_cards(pool: &SqlitePool) -> Result<Vec<Card>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM cards").fetch_all(pool).await
}
