use crate::DbPool;
use crate::module::health::model::HealthState;

pub async fn check_database(pool: &DbPool) -> Result<HealthState, sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(HealthState::Healthy)
}
