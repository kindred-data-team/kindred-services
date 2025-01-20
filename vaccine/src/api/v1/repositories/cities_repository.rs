use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::cities_model::{City, CreateCity};

pub async fn get_cities(pool: &Pool<MySql>) -> Result<Vec<City>, Error> {
    sqlx::query_as!(
        City,
        r#"
        SELECT *
        FROM cities
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_city_by_id(pool: &Pool<MySql>, id: i32) -> Result<City, Error> {
    sqlx::query_as!(
        City,
        r#"
        SELECT *
        FROM cities
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn add_city(pool: &Pool<MySql>, city: CreateCity) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO cities (
            name, created_at, updated_at
        )
        VALUES (?, NOW(), NOW())
        "#,
        city.name
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_city(pool: &Pool<MySql>, id: i32) -> Result<Option<i32>, Error> {

    let data = sqlx::query!{
        r#"
        SELECT id
        FROM cities
        WHERE id = ?
        "#,
        id
    }
    .fetch_optional(pool)
    .await?;

    if data.is_none() {
        return Ok(None);
    }

    sqlx::query!(
        r#"
        DELETE FROM cities
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(Some(id))
}