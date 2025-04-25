#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
use sea_orm::{DbBackend, FromQueryResult, Statement, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
struct Report
{
    total_stock: i64,
    total_stock_value: Decimal,
    average_price: Decimal
}

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response>
{
    let report: Option<Report> = Report::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    SUM(quantity) AS total_stock,
                    SUM(price*quantity) AS total_stock_value,
                    AVG(price) AS average_price
                FROM
                    items
            "#,
            []
        ))
        .one(&ctx.db)
        .await?;

    format::json(report)
}

pub async fn reports_by_category(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response>
{
    let filtered = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT
                    id,
                    name,
                    description,
                    price,
                    quantity
                FROM
                    items
                WHERE
                    category_id = $1
            "#,
            [id.into()]
        ))
        .all(&ctx.db)
        .await?;

    format::json(filtered)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/reports/")
        .add("/", get(index))
        .add("/category/{id}", get(reports_by_category))
}
