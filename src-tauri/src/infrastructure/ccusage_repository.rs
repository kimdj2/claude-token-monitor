use std::process::Command;
use std::path::PathBuf;
use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;
use async_trait::async_trait;
use crate::domain::{
  entities::{UsageStats, UsagePeriodSummary},
  repository::UsageRepository,
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TokenCounts {
  #[serde(rename = "inputTokens")]
  input_tokens: u32,
  #[serde(rename = "outputTokens")]
  output_tokens: u32,
  #[serde(rename = "cacheCreationInputTokens")]
  cache_creation_input_tokens: u32,
  #[serde(rename = "cacheReadInputTokens")]
  cache_read_input_tokens: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct BurnRate {
  #[serde(rename = "tokensPerMinute")]
  tokens_per_minute: f32,
  #[serde(rename = "tokensPerMinuteForIndicator")]
  tokens_per_minute_for_indicator: f32,
  #[serde(rename = "costPerHour")]
  cost_per_hour: f32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Projection {
  #[serde(rename = "totalTokens")]
  total_tokens: u32,
  #[serde(rename = "totalCost")]
  total_cost: f32,
  #[serde(rename = "remainingMinutes")]
  remaining_minutes: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct CcusageBlock {
  id: String,
  #[serde(rename = "startTime")]
  start_time: String,
  #[serde(rename = "endTime")]
  end_time: String,
  #[serde(rename = "actualEndTime")]
  actual_end_time: Option<String>,
  #[serde(rename = "isActive")]
  is_active: bool,
  #[serde(rename = "isGap")]
  is_gap: bool,
  entries: u32,
  #[serde(rename = "tokenCounts")]
  token_counts: TokenCounts,
  #[serde(rename = "totalTokens")]
  total_tokens: u32,
  #[serde(rename = "costUSD")]
  cost_usd: f32,
  models: Vec<String>,
  #[serde(rename = "burnRate")]
  burn_rate: Option<BurnRate>,
  projection: Option<Projection>,
}

#[derive(Deserialize, Debug)]
struct CcusageBlocksResponse {
  blocks: Vec<CcusageBlock>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct CcusageDailyEntry {
  date: String,
  #[serde(rename = "totalTokens")]
  total_tokens: u32,
  #[serde(rename = "totalCost")]
  total_cost: f32,
  #[serde(rename = "modelsUsed")]
  models_used: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CcusageDailyResponse {
  daily: Vec<CcusageDailyEntry>,
}

pub struct CcusageRepository;

impl CcusageRepository {
  pub fn new() -> Self {
    Self
  }

  fn find_node_and_ccusage_paths(&self) -> Result<(String, String), String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Could not get HOME directory")?;

    // Node.js path candidates
    let mut node_candidates = vec![
      "/opt/homebrew/bin/node".to_string(),  // Homebrew Apple Silicon
      "/usr/local/bin/node".to_string(),     // Homebrew Intel, official install
      format!("{}/.volta/bin/node", home_dir), // Volta
      "/usr/bin/node".to_string(),           // System
    ];

    // Dynamically add nvm installation paths
    if let Ok(entries) = std::fs::read_dir(format!("{}/.nvm/versions/node", home_dir)) {
      for entry in entries.flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
          node_candidates.push(format!("{}/.nvm/versions/node/{}/bin/node", home_dir, file_name));
        }
      }
    }

    // Dynamically add fnm installation paths
    if let Ok(entries) = std::fs::read_dir(format!("{}/.local/share/fnm/node-versions", home_dir)) {
      for entry in entries.flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
          node_candidates.push(format!("{}/.local/share/fnm/node-versions/{}/installation/bin/node", home_dir, file_name));
        }
      }
    }

    // Dynamically add asdf installation paths
    if let Ok(entries) = std::fs::read_dir(format!("{}/.asdf/installs/nodejs", home_dir)) {
      for entry in entries.flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
          node_candidates.push(format!("{}/.asdf/installs/nodejs/{}/bin/node", home_dir, file_name));
        }
      }
    }

    // ccusage path candidates
    let ccusage_candidates = vec![
      "/opt/homebrew/bin/ccusage".to_string(),  // Homebrew Apple Silicon
      "/usr/local/bin/ccusage".to_string(),     // Homebrew Intel, npm global
      format!("{}/.yarn/bin/ccusage", home_dir), // Yarn global
      format!("{}/.local/share/pnpm/ccusage", home_dir), // pnpm global
    ];

    // Find Node.js path
    let node_path = if let Ok(node_env) = std::env::var("NODE_PATH") {
      println!("🔍 Using NODE_PATH environment variable: {}", node_env);
      node_env
    } else {
      println!("🔍 Searching for Node.js in {} candidates", node_candidates.len());
      let found_node = node_candidates.into_iter()
        .find(|path| {
          let exists = PathBuf::from(path).exists();
          println!("🔍 Checking Node.js path: {} -> {}", path, exists);
          exists
        })
        .unwrap_or_else(|| {
          println!("⚠️ No Node.js found in candidates, falling back to system PATH");
          "node".to_string()
        });
      found_node
    };

    // Find ccusage path
    let ccusage_path = if let Ok(ccusage_env) = std::env::var("CCUSAGE_PATH") {
      println!("🔍 Using CCUSAGE_PATH environment variable: {}", ccusage_env);
      ccusage_env
    } else {
      println!("🔍 Searching for ccusage in {} candidates", ccusage_candidates.len());
      let found_ccusage = ccusage_candidates.into_iter()
        .find(|path| {
          let exists = PathBuf::from(path).exists();
          println!("🔍 Checking ccusage path: {} -> {}", path, exists);
          exists
        })
        .unwrap_or_else(|| {
          println!("⚠️ No ccusage found in candidates, falling back to system PATH");
          "ccusage".to_string()
        });
      found_ccusage
    };

    println!("✅ Selected paths - Node.js: {}, ccusage: {}", node_path, ccusage_path);
    Ok((node_path, ccusage_path))
  }

  fn create_command_with_env(&self, ccusage_path: &str, node_path: &str) -> Command {
    let mut cmd = Command::new(ccusage_path);

    // Configure PATH including dynamically found paths
    let current_path = std::env::var("PATH").unwrap_or_default();
    let node_pathbuf = PathBuf::from(node_path);
    let node_dir = node_pathbuf.parent()
      .and_then(|p| p.to_str())
      .unwrap_or("/usr/local/bin");

    let enhanced_path = format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:{}",
                               node_dir, current_path);

    cmd.env("PATH", enhanced_path);
    cmd.env("NODE_PATH", node_path);
    cmd
  }

  fn create_user_friendly_error(&self, ccusage_path: &str, node_path: &str, error: &std::io::Error) -> String {
    match error.kind() {
      std::io::ErrorKind::NotFound => {
        if ccusage_path == "ccusage" {
          format!("❌ ccusage not found\n\n📋 To use Claude Token Monitor, you need to install ccusage:\n\n1️⃣ Install ccusage globally:\n   npm install -g ccusage\n\n2️⃣ Or using yarn:\n   yarn global add ccusage\n\n3️⃣ Or using pnpm:\n   pnpm add -g ccusage\n\n4️⃣ Make sure ccusage is in your PATH\n\n5️⃣ Restart Claude Token Monitor after installation\n\n💡 Alternative: Set CCUSAGE_PATH environment variable to custom installation path")
        } else if node_path == "node" {
          format!("❌ Node.js not found\n\n📋 To use Claude Token Monitor, you need Node.js:\n\n1️⃣ Install Node.js from https://nodejs.org\n\n2️⃣ Or using Homebrew:\n   brew install node\n\n3️⃣ Or using a version manager:\n   • nvm: https://github.com/nvm-sh/nvm\n   • fnm: https://github.com/Schniz/fnm\n   • volta: https://volta.sh\n\n4️⃣ After installation, install ccusage:\n   npm install -g ccusage\n\n💡 Alternative: Set NODE_PATH environment variable to custom Node.js path")
        } else {
          format!("❌ Command execution failed\n\n🔍 Detected paths:\n• Node.js: {}\n• ccusage: {}\n\n📋 Troubleshooting:\n\n1️⃣ Verify Node.js installation:\n   {} --version\n\n2️⃣ Verify ccusage installation:\n   {} --version\n\n3️⃣ Check file permissions\n\n4️⃣ Try reinstalling ccusage:\n   npm install -g ccusage\n\n💡 Error details: {}", node_path, ccusage_path, node_path, ccusage_path, error)
        }
      }
      std::io::ErrorKind::PermissionDenied => {
        format!("❌ Permission denied\n\n🔐 File permission issue detected:\n\n📋 Solutions:\n\n1️⃣ Check file permissions:\n   ls -la {}\n   ls -la {}\n\n2️⃣ Try reinstalling with proper permissions:\n   sudo npm install -g ccusage\n\n3️⃣ Or use a Node version manager (recommended):\n   • nvm: No sudo required\n   • fnm: No sudo required\n   • volta: No sudo required\n\n💡 Error details: {}", node_path, ccusage_path, error)
      }
      _ => {
        format!("❌ Unexpected error occurred\n\n🔍 System information:\n• Node.js path: {}\n• ccusage path: {}\n• Error type: {:?}\n\n📋 Troubleshooting steps:\n\n1️⃣ Restart the application\n\n2️⃣ Reinstall ccusage:\n   npm install -g ccusage\n\n3️⃣ Check system resources (disk space, memory)\n\n4️⃣ Try running from terminal to see detailed errors\n\n💡 Error details: {}", node_path, ccusage_path, error.kind(), error)
      }
    }
  }

  fn create_ccusage_command_error(&self, command: &str, stderr: &str) -> String {
    if stderr.contains("command not found") || stderr.contains("No such file") {
      format!("❌ ccusage command not found\n\n📋 Installation required:\n\n1️⃣ Install ccusage globally:\n   npm install -g ccusage\n\n2️⃣ Verify installation:\n   ccusage --version\n\n3️⃣ Restart Claude Token Monitor\n\n💡 If using yarn or pnpm:\n   yarn global add ccusage\n   pnpm add -g ccusage")
    } else if stderr.contains("ENOENT") {
      format!("❌ Node.js or ccusage not accessible\n\n🔍 This usually means:\n• Node.js is not installed\n• ccusage is not installed\n• PATH environment variable issue\n\n📋 Quick fix:\n\n1️⃣ Install Node.js: https://nodejs.org\n2️⃣ Install ccusage: npm install -g ccusage\n3️⃣ Restart the application\n\n💡 Error: {}", stderr)
    } else if stderr.contains("permission") || stderr.contains("EACCES") {
      format!("❌ Permission denied\n\n🔐 ccusage execution blocked by permissions\n\n📋 Solutions:\n\n1️⃣ Reinstall with sudo (not recommended):\n   sudo npm install -g ccusage\n\n2️⃣ Use Node version manager (recommended):\n   • Install nvm, fnm, or volta\n   • No sudo required\n\n3️⃣ Fix npm permissions:\n   npm config set prefix ~/.npm-global\n\n💡 Error: {}", stderr)
    } else if stderr.contains("Claude Code") || stderr.contains("session") {
      format!("❌ Claude Code session issue\n\n🔍 ccusage can't access Claude data:\n\n📋 Check these:\n\n1️⃣ Is Claude Code (VS Code/Cursor extension) installed?\n2️⃣ Have you used Claude recently?\n3️⃣ Are you logged into Claude?\n4️⃣ Try using Claude once, then refresh\n\n💡 ccusage {} error: {}", command, stderr)
    } else {
      format!("❌ ccusage {} command failed\n\n🔍 Unexpected error occurred:\n\n📋 Troubleshooting:\n\n1️⃣ Try updating ccusage:\n   npm update -g ccusage\n\n2️⃣ Check ccusage version:\n   ccusage --version\n\n3️⃣ Test ccusage manually:\n   ccusage {}\n\n4️⃣ Reinstall if needed:\n   npm uninstall -g ccusage\n   npm install -g ccusage\n\n💡 Raw error: {}", command, command, stderr)
    }
  }
}

#[async_trait]
impl UsageRepository for CcusageRepository {
  async fn get_claude_usage(&self) -> Result<UsageStats, String> {
      let (node_path, ccusage_path) = self.find_node_and_ccusage_paths()?;
      let blocks_output = self.create_command_with_env(&ccusage_path, &node_path)
      .args(&["blocks", "--json"])
      .output()
      .map_err(|e| self.create_user_friendly_error(&ccusage_path, &node_path, &e))?;

      if !blocks_output.status.success() {
      let stderr = String::from_utf8_lossy(&blocks_output.stderr);
      return Err(self.create_ccusage_command_error("blocks", &stderr));
      }

      let blocks_json = String::from_utf8_lossy(&blocks_output.stdout);
      let blocks_response: CcusageBlocksResponse = serde_json::from_str(&blocks_json)
      .map_err(|e| format!("Failed to parse ccusage blocks output: {}", e))?;

      let active_block = blocks_response.blocks.iter().find(|block| block.is_active);

      let (current_tokens, session_cost, burn_rate_value, primary_model) = match active_block {
      Some(block) => (
          block.total_tokens,
          block.cost_usd,
          block.burn_rate.as_ref().map(|br| br.tokens_per_minute_for_indicator),
          block.models.first().cloned().unwrap_or_else(|| "Unknown".to_string()),
      ),
      None => (0, 0.0, None, blocks_response.blocks.first()
          .and_then(|recent| recent.models.first().cloned())
          .unwrap_or_else(|| "Claude".to_string())),
      };

      let daily_output = self.create_command_with_env(&ccusage_path, &node_path)
        .args(&["daily", "--json"])
        .output()
        .map_err(|e| self.create_user_friendly_error(&ccusage_path, &node_path, &e))?;

      if !daily_output.status.success() {
      let stderr = String::from_utf8_lossy(&daily_output.stderr);
      return Err(self.create_ccusage_command_error("daily", &stderr));
      }

      let daily_json = String::from_utf8_lossy(&daily_output.stdout);
      let daily_response: CcusageDailyResponse = serde_json::from_str(&daily_json)
      .map_err(|e| format!("Failed to parse ccusage daily output: {}", e))?;
      let today = Local::now().date_naive();
      let (daily_tokens, daily_cost) = daily_response.daily.iter()
      .find(|entry| NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d").ok() == Some(today))
      .map_or((0, 0.0), |entry| (entry.total_tokens, entry.total_cost));

      Ok(UsageStats {
      active_session: active_block.is_some(),
      current_tokens,
      daily_tokens,
      cost: daily_cost,
      model: primary_model,
      session_cost,
      burn_rate: burn_rate_value,
      })
  }

  async fn get_usage_summary(&self, period: String) -> Result<UsagePeriodSummary, String> {
      let (node_path, ccusage_path) = self.find_node_and_ccusage_paths()?;
      let daily_output = self.create_command_with_env(&ccusage_path, &node_path)
        .args(&["daily", "--json"])
        .output()
        .map_err(|e| self.create_user_friendly_error(&ccusage_path, &node_path, &e))?;

      if !daily_output.status.success() {
      let stderr = String::from_utf8_lossy(&daily_output.stderr);
      return Err(self.create_ccusage_command_error("daily", &stderr));
      }

      let daily_json = String::from_utf8_lossy(&daily_output.stdout);
      let daily_response: CcusageDailyResponse = serde_json::from_str(&daily_json)
      .map_err(|e| format!("daily JSON parsing failed: {}", e))?;
      let today = Local::now().date_naive();
      let (start, end, days) = match period.as_str() {
      "week" => {
          // Last 7 days including today
          let start = today.checked_sub_days(chrono::Days::new(6)).unwrap_or(today);
          (start, today, 7)
      }
      "month" => {
          let start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).ok_or("invalid month")?;
          let end = today;
          let days = (end - start).num_days() as u32 + 1;
          (start, end, days)
      }
      _ => (today, today, 1), // Always use today for day period
      };

      let mut total_tokens = 0;
      let mut total_cost = 0.0;
      for entry in &daily_response.daily {
      if let Ok(entry_date) = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
          if entry_date >= start && entry_date <= end {
        total_tokens += entry.total_tokens;
        total_cost += entry.total_cost;
          }
      }
      }

      let avg_tokens_per_day = if days > 0 { total_tokens as f32 / days as f32 } else { 0.0 };
      let avg_cost_per_day = if days > 0 { total_cost / days as f32 } else { 0.0 };

      Ok(UsagePeriodSummary {
      period,
      start_date: start.format("%Y-%m-%d").to_string(),
      end_date: end.format("%Y-%m-%d").to_string(),
      days,
      total_tokens,
      total_cost,
      avg_tokens_per_day,
      avg_cost_per_day,
      })
  }
}
