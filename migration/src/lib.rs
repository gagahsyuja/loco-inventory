#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250423_144100_admins;
mod m20250423_144122_suppliers;
mod m20250423_144419_categories;
mod m20250423_144850_items;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250423_144100_admins::Migration),
            Box::new(m20250423_144122_suppliers::Migration),
            Box::new(m20250423_144419_categories::Migration),
            Box::new(m20250423_144850_items::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
