use std::sync::Arc;
use crate::domain::{
  entities::{UsageStats, UsagePeriodSummary},
  repository::UsageRepository,
};

pub async fn get_claude_usage(repo: Arc<dyn UsageRepository + Send + Sync>) -> Result<UsageStats, String> {
  repo.get_claude_usage().await
}

pub async fn get_usage_summary(repo: Arc<dyn UsageRepository + Send + Sync>, period: String) -> Result<UsagePeriodSummary, String> {
  repo.get_usage_summary(period).await
}