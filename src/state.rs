use std::sync::Arc;

use crate::model::Config;
use anyhow::Result;
use casbin::{CoreApi, Enforcer};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub redis_client: redis::Client,
    pub config: Config,
    pub enforcer: Arc<Enforcer>,
}

impl AppState {
    pub async fn init(cfg: &Config) -> Result<AppState> {
        let pg_pool = Self::init_database(cfg).await?;
        let redis_client = Self::init_redis(cfg).await?;
        let enforcer = Self::init_casbin().await?;

        Ok(Self {
            pg_pool,
            redis_client,
            config: cfg.clone(),
            enforcer,
        })
    }

    // init database
    pub async fn init_database(cfg: &Config) -> Result<PgPool> {
        let db_connection_str = cfg.database_url.clone();

        // setup connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_connection_str)
            .await?;

        tracing::info!("database ok");
        Ok(pool)
    }

    // init redis
    pub async fn init_redis(cfg: &Config) -> Result<redis::Client> {
        let redis_connection_str = cfg.redis_url.clone();

        let client = redis::Client::open(redis_connection_str)?;

        tracing::info!("redis ok");

        Ok(client)
    }

    pub async fn init_casbin() -> Result<Arc<Enforcer>> {
        let e = Enforcer::new("conf/keymatch_model.conf", "conf/keymatch_policy.csv").await?;
        Ok(Arc::new(e))
    }
}
