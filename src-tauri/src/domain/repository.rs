use async_trait::async_trait;
use super::entities::{UsageStats, UsagePeriodSummary};

#[async_trait]
pub trait UsageRepository {
  async fn get_claude_usage(&self) -> Result<UsageStats, String>;
  async fn get_usage_summary(&self, period: String) -> Result<UsagePeriodSummary, String>;
}