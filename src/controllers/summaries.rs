#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
use sea_orm::{ Statement, DbBackend, FromQueryResult, JsonValue };

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response>
{
    let summary = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    COUNT(items.id) AS total_item_counts,
                    SUM(items.price*items.quantity) AS total_item_values,
                    (SELECT COUNT(id) FROM categories) AS total_category_counts,
                    (SELECT COUNT(id) FROM suppliers) AS total_supplier_counts
                FROM items;
            "#,
            []
        ))
        .all(&ctx.db)
        .await?;

    format::json(summary)
}

#[debug_handler]
pub async fn summary_by_all_category(State(ctx): State<AppContext>) -> Result<Response>
{
    let summary = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    categories.id AS category_id,
                    categories.name AS category_name,
                    SUM(items.quantity) AS total_quantity,
                    SUM(items.price*items.quantity) AS total_price_value 
                FROM
                    items
                JOIN
                    categories ON items.category_id = categories.id
                GROUP BY
                    categories.id
                ORDER BY
                    categories.id;
            "#,
            []
        ))
        .all(&ctx.db)
        .await?;

    format::json(summary)
}

#[debug_handler]
pub async fn summary_by_category(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response>
{
    let summary = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    categories.id AS category_id,
                    categories.name AS category_name,
                    SUM(items.quantity) AS total_quantity,
                    SUM(items.price*items.quantity) AS total_price_value 
                FROM
                    items
                JOIN
                    categories ON items.category_id = categories.id
                WHERE
                    categories.id = $1
                GROUP BY
                    categories.id;
            "#,
            [id.into()]
        ))
        .one(&ctx.db)
        .await?;

    format::json(summary)
}

pub async fn summary_by_all_supplier(State(ctx): State<AppContext>) -> Result<Response>
{
    let summary = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    suppliers.id AS supplier_id,
                    suppliers.name AS supplier_name,
                    SUM(items.quantity) AS total_quantity,
                    SUM(items.price * items.quantity) AS total_price_value 
                FROM
                    items
                JOIN
                    suppliers ON items.supplier_id = suppliers.id
                GROUP BY
                    suppliers.id
                ORDER BY
                    suppliers.id;
            "#,
            []
        ))
        .all(&ctx.db)
        .await?;

    format::json(summary)
}

#[debug_handler]
pub async fn summary_by_supplier(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response>
{
    let summary = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    suppliers.id AS supplier_id,
                    suppliers.name AS supplier_name,
                    SUM(items.quantity) AS total_quantity,
                    SUM(items.price * items.quantity) AS total_price_value 
                FROM
                    items
                JOIN
                    suppliers ON items.supplier_id = suppliers.id
                WHERE
                    suppliers.id = $1
                GROUP BY
                    suppliers.id;
            "#,
            [id.into()]
        ))
        .one(&ctx.db)
        .await?;

    format::json(summary)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/summaries/")
        .add("/", get(index))
        .add("/category", get(summary_by_all_category))
        .add("/category/{id}", get(summary_by_category))
        .add("/supplier", get(summary_by_all_supplier))
        .add("/supplier/{id}", get(summary_by_supplier))
}
