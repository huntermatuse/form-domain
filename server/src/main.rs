use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{EnvFilter, fmt};

use server::config::Config;
use server::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = Config::parse();

    let database_url = config.resolved_database_url();
    tracing::info!("using database: {}", database_url);

    tracing::trace!("connecting to database");
    let db = PgPoolOptions::new()
        .connect(database_url)
        .await
        .context("could not connect to database_url")?;
    tracing::trace!("database connection established");

    tracing::trace!("running migrations");
    sqlx::migrate!().run(&db).await?;
    tracing::trace!("migrations complete");

    http::serve(config, db).await?;

    Ok(())
}
