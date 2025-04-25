use async_trait::async_trait;
use axum::Router;
use tower_http::normalize_path::NormalizePathLayer;

use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};
use crate::models::_entities::admins;
use crate::models::_entities::categories;
use crate::models::_entities::suppliers;
use crate::models::_entities::items;

#[allow(clippy::module_name_repetitions)]
pub struct NormalizePathInitializer;

#[async_trait]
impl Initializer for NormalizePathInitializer
{
    fn name(&self) -> String {
        "normalize-path".to_string()
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        let router = router.layer(NormalizePathLayer::trim_trailing_slash());
        Ok(router)
    }
}

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(NormalizePathInitializer)])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::summaries::routes())
            .add_route(controllers::report::routes())
            .add_route(controllers::admins::routes())
            .add_route(controllers::items::routes())
            .add_route(controllers::categories::routes())
            .add_route(controllers::suppliers::routes())
            .add_route(controllers::auth::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<admins::ActiveModel>(&ctx.db, &base.join("admins.yaml").display().to_string())
            .await?;
        db::seed::<categories::ActiveModel>(&ctx.db, &base.join("categories.yaml").display().to_string())
            .await?;
        db::seed::<suppliers::ActiveModel>(&ctx.db, &base.join("suppliers.yaml").display().to_string())
            .await?;
        db::seed::<items::ActiveModel>(&ctx.db, &base.join("items.yaml").display().to_string())
            .await?;
        Ok(())
    }
}