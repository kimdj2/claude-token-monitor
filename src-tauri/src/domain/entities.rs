use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageStats {
  pub active_session: bool,
  pub current_tokens: u32,
  pub daily_tokens: u32,
  pub cost: f32,
  pub model: String,
  pub session_cost: f32,
  pub burn_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsagePeriodSummary {
  pub period: String,
  pub start_date: String,
  pub end_date: String,
  pub days: u32,
  pub total_tokens: u32,
  pub total_cost: f32,
  pub avg_tokens_per_day: f32,
  pub avg_cost_per_day: f32,
}
