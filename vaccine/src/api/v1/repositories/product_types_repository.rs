use sqlx::{MySql, Pool, Error};
use crate::api::v1::models::{product_types_model::*, response_model::Meta};

pub async fn get_product_types(
    pool: &Pool<MySql>, 
    page: usize, 
    limit: usize
) -> Result<PaginatedProductTypes, Error> {
    // Get total count
    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM product_types",
    )
    .fetch_one(pool)
    .await?;

    let offset = (page - 1) * limit;
    let product_types = sqlx::query_as!(
        ProductType,
        r#"
        SELECT *
        FROM product_types
        ORDER BY id LIMIT ? OFFSET ? 
        "#,
        limit as i32,
        offset as i32
    )
    .fetch_all(pool)
    .await?;

    Ok(PaginatedProductTypes {
        data: product_types,
        meta: Meta {
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as usize
        }
    })
}

pub async fn get_product_type_by_id(
    pool: &Pool<MySql>,
     id: i32
) -> Result<ProductType, Error> {
    sqlx::query_as!(
        ProductType,
        r#"
        SELECT *
        FROM product_types
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn add_product_type(
    pool: &Pool<MySql>, 
    product_type: &CreateProductType, 
    user: i32
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO product_types (
            name, created_by, updated_by, created_at, updated_at
        )
        VALUES (?, ?, ?, NOW(), NOW())
        "#,
        product_type.name,
        user,
        user
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn edit_product_type(
    pool: &Pool<MySql>, 
    id: i32, 
    product_type: &EditProductType, 
    user_id: i32
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE product_types
        SET name = ?, updated_by = ?, updated_at = NOW()
        WHERE id = ?
        "#,
        product_type.name,
        user_id,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_product_type(
    pool: &Pool<MySql>,
    id: i32, 
    user_id: i32
) -> Result<Option<i32>, Error> {
    let data = sqlx::query!{
        r#"
        SELECT id
        FROM product_types
        WHERE (id = ? AND created_by = ?)
        "#,
        id,
        user_id
    }
    .fetch_optional(pool)
    .await?;

    if data.is_none() {
        return Ok(None);
    }

    sqlx::query!(
        r#"
        DELETE FROM product_types
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(Some(id))
}