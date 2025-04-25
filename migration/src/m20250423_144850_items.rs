use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "items",
            &[
            
            ("id", ColType::PkAuto),
            
            ("name", ColType::TextNull),
            ("description", ColType::TextNull),
            ("price", ColType::DecimalNull),
            ("quantity", ColType::IntegerNull),
            ],
            &[
            ("categories", "category_id"),
            ("suppliers", "supplier_id"),
            ("admins", "created_by"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "items").await
    }
}
