export interface UsageStats {
  active_session: boolean;
  current_tokens: number;
  daily_tokens: number;
  cost: number;
  model: string;
  session_cost: number;
  burn_rate: number | null;
}

export interface UsageSummary {
  period: string;
  start_date: string;
  end_date: string;
  days: number;
  total_tokens: number;
  total_cost: number;
  avg_tokens_per_day: number;
  avg_cost_per_day: number;
}

export type Period = 'day' | 'week' | 'month';

export type WarningLevel = 'safe' | 'warning' | 'critical' | 'danger';

export interface UsagePattern {
  averageDailyUsage: number;
  peakUsageHours: number[];
  typicalBurnRate: number;
  maxDailyUsage: number;
  consistencyScore: number;
}
