#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use sea_orm::{DbBackend, FromQueryResult, Statement, JsonValue};
use serde::{Deserialize, Serialize};
use axum::{debug_handler, extract::Path};

use crate::models::_entities::items::{ActiveModel, Entity, Model, Column};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub quantity: Option<i32>,
    }

impl Params {
    fn update(&self, item: &mut ActiveModel) {
      item.name = Set(self.name.clone());
      item.description = Set(self.description.clone());
      item.price = Set(self.price);
      item.quantity = Set(self.quantity);
      }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
struct Report
{
    total_stock: i64,
    total_stock_value: Decimal,
    average_price: Decimal
}

#[debug_handler]
pub async fn report(State(ctx): State<AppContext>) -> Result<Response>
{
    let report: Option<Report> = Report::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT SUM(quantity) AS total_stock, SUM(price*quantity) AS total_stock_value, AVG(price) AS average_price FROM items"#,
            []
        ))
        .one(&ctx.db)
        .await?;

    format::json(report)
}

#[debug_handler]
pub async fn filter_by_quantity(State(ctx): State<AppContext>, Path(number): Path<i32>) -> Result<Response>
{
    let filtered = Entity::find()
        .filter(Column::Quantity.lte(number))
        .all(&ctx.db)
        .await?;

    format::json(filtered)
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
struct Summary
{
    category_id: i32,
    category_name: String,
    total_quantity: i64,
    total_price_value: Decimal
}

#[debug_handler]
pub async fn filter_by_category(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response>
{
    let filtered = Entity::find()
        .filter(Column::CategoryId.eq(id))
        .all(&ctx.db)
        .await?;

    format::json(filtered)
}

#[debug_handler]
pub async fn summary_by_category(State(ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response>
{
    let summary = Summary::find_by_statement(Statement::from_sql_and_values(
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
        .prefix("api/items/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
        // .add("/report", get(report))
        .add("/filter-by-quantity/{number}", get(filter_by_quantity))
        .add("/filter-by-category/{id}", get(filter_by_category))
}
