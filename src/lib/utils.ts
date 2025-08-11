import type { Period, UsageStats, UsageSummary } from './types';

export function formatTokens(tokens: number): string {
  return tokens.toLocaleString();
}

export function formatCost(cost: number): string {
  return `$${cost.toFixed(3)}`;
}

export function getPeriodTokens(period: Period, usageStats: UsageStats | null, summary: UsageSummary | null): number {
  if (!usageStats) return 0;
  if (period === 'day') {
    return usageStats.daily_tokens;
  } else if (summary) {
    return summary.total_tokens;
  } else {
    return usageStats.daily_tokens;
  }
}

export function getMaxTokens(period: Period): number {
  switch (period) {
    case 'day': return 1000000;
    case 'week': return 7000000;
    case 'month': return 30000000;
    default: return 1000000;
  }
}

export function getMaxTokensLabel(period: Period): string {
  switch (period) {
    case 'day': return '1M';
    case 'week': return '7M';
    case 'month': return '30M';
    default: return '1M';
  }
}

export function getPeriodLabel(period: Period): string {
  switch (period) {
    case 'day': return 'Today';
    case 'week': return 'This Week';
    case 'month': return 'This Month';
    default: return 'Today';
  }
}

export function getPeriodCost(period: Period, usageStats: UsageStats | null, summary: UsageSummary | null): number {
  if (!usageStats) return 0;
  if (period === 'day') {
    return usageStats.cost;
  } else if (summary) {
    return summary.total_cost;
  } else {
    return usageStats.cost;
  }
}

// Warning system functions
export function getUsagePercentage(period: Period, usageStats: UsageStats | null, summary: UsageSummary | null): number {
  const tokens = getPeriodTokens(period, usageStats, summary);
  const maxTokens = getMaxTokens(period);
  return (tokens / maxTokens) * 100;
}

export type WarningLevel = 'safe' | 'warning' | 'critical' | 'danger';

export function getWarningLevel(percentage: number): WarningLevel {
  if (percentage >= 95) return 'danger';
  if (percentage >= 85) return 'critical';
  if (percentage >= 70) return 'warning';
  return 'safe';
}

export function getWarningColor(level: WarningLevel): string {
  switch (level) {
    case 'danger': return '#ff4757';    // Red
    case 'critical': return '#ff6b35';  // Orange-red
    case 'warning': return '#ffa726';   // Orange
    case 'safe': return '#4fc3f7';      // Blue
  }
}

export function getWarningMessage(level: WarningLevel, percentage: number): string | null {
  switch (level) {
    case 'danger': 
      return `⚠️ Critical: ${percentage.toFixed(1)}% usage! Consider upgrading your plan.`;
    case 'critical': 
      return `🚨 High usage: ${percentage.toFixed(1)}%. Monitor your token consumption.`;
    case 'warning': 
      return `⚡ Moderate usage: ${percentage.toFixed(1)}%. Keep an eye on usage.`;
    default: 
      return null;
  }
}

// Advanced warning system based on Claude Code monitor approach
export function calculateTimeToLimit(
  currentTokens: number, 
  maxTokens: number, 
  burnRate: number | null
): string | null {
  if (!burnRate || burnRate <= 0) return null;
  
  const remainingTokens = maxTokens - currentTokens;
  if (remainingTokens <= 0) return "Limit reached";
  
  const minutesRemaining = remainingTokens / burnRate;
  
  if (minutesRemaining < 1) {
    return "< 1 min";
  } else if (minutesRemaining < 60) {
    return `${Math.round(minutesRemaining)} min`;
  } else if (minutesRemaining < 1440) {
    const hours = Math.round(minutesRemaining / 60);
    return `${hours}h`;
  } else {
    const days = Math.round(minutesRemaining / 1440);
    return `${days}d`;
  }
}

export function getAdvancedWarningMessage(
  percentage: number,
  timeToLimit: string | null,
  burnRate: number | null
): string | null {
  if (percentage >= 95) {
    if (timeToLimit && timeToLimit !== "Limit reached") {
      return `🚨 Critical: ${percentage.toFixed(1)}% used. Approx. ${timeToLimit} remaining at current rate.`;
    }
    return `🚨 Critical: ${percentage.toFixed(1)}% used. Limit almost reached!`;
  }
  
  if (percentage >= 85) {
    if (timeToLimit) {
      return `⚠️ High usage: ${percentage.toFixed(1)}% used. Approx. ${timeToLimit} remaining.`;
    }
    return `⚠️ High usage: ${percentage.toFixed(1)}% used. Monitor consumption closely.`;
  }
  
  if (percentage >= 70) {
    if (burnRate && burnRate > 0) {
      return `⚡ Moderate usage: ${percentage.toFixed(1)}% used. Current rate: ${burnRate.toFixed(1)}/min.`;
    }
    return `⚡ Moderate usage: ${percentage.toFixed(1)}% used. Track your consumption.`;
  }
  
  return null;
}

export function shouldShowUrgentWarning(percentage: number, timeToLimit: string | null): boolean {
  if (percentage >= 95) return true;
  if (timeToLimit === "< 1 min" || timeToLimit === "Limit reached") return true;
  return false;
}

export function getBurnRateColor(burnRate: number | null): string {
  if (!burnRate) return '#6b7280'; // Gray
  
  if (burnRate > 100) return '#ef4444'; // High burn rate - red
  if (burnRate > 50) return '#f59e0b';  // Medium burn rate - orange  
  if (burnRate > 20) return '#eab308';  // Low-medium burn rate - yellow
  return '#10b981'; // Low burn rate - green
}

export function formatBurnRate(burnRate: number | null): string {
  if (!burnRate) return 'No data';
  
  if (burnRate >= 1000) {
    return `${(burnRate / 1000).toFixed(1)}K/min`;
  }
  return `${burnRate.toFixed(1)}/min`;
}

// Adaptive thresholds based on usage patterns
export interface UsagePattern {
  averageDailyUsage: number;
  peakUsageHours: number[];
  typicalBurnRate: number;
  maxDailyUsage: number;
  consistencyScore: number; // 0-1, higher = more consistent usage
}

export function analyzeUsagePattern(
  weeklyData: { date: string; tokens: number; cost: number }[]
): UsagePattern {
  if (weeklyData.length === 0) {
    return {
      averageDailyUsage: 0,
      peakUsageHours: [],
      typicalBurnRate: 0,
      maxDailyUsage: 0,
      consistencyScore: 0
    };
  }

  const dailyUsages = weeklyData.map(d => d.tokens);
  const averageDailyUsage = dailyUsages.reduce((sum, usage) => sum + usage, 0) / dailyUsages.length;
  const maxDailyUsage = Math.max(...dailyUsages);
  
  // Calculate consistency score (lower standard deviation = higher consistency)
  const variance = dailyUsages.reduce((sum, usage) => sum + Math.pow(usage - averageDailyUsage, 2), 0) / dailyUsages.length;
  const stdDev = Math.sqrt(variance);
  const consistencyScore = Math.max(0, 1 - (stdDev / averageDailyUsage));

  return {
    averageDailyUsage,
    peakUsageHours: [9, 10, 11, 14, 15, 16], // Default business hours
    typicalBurnRate: averageDailyUsage / (8 * 60), // Assume 8 hours of work
    maxDailyUsage,
    consistencyScore: isNaN(consistencyScore) ? 0 : consistencyScore
  };
}

export function getAdaptiveThresholds(pattern: UsagePattern, _period: Period): { warning: number; critical: number; danger: number } {
  const baseThresholds = { warning: 70, critical: 85, danger: 95 };
  
  if (pattern.averageDailyUsage === 0) {
    return baseThresholds;
  }

  // Adjust thresholds based on usage patterns
  let adjustment = 0;
  
  // Heavy users get earlier warnings
  if (pattern.averageDailyUsage > 500000) { // 500K+ daily
    adjustment = -10; // Earlier warnings
  } else if (pattern.averageDailyUsage > 200000) { // 200K+ daily
    adjustment = -5;
  }
  
  // Consistent users get slightly later warnings (they know their usage)
  if (pattern.consistencyScore > 0.8) {
    adjustment += 5;
  }
  
  // High burn rate users get earlier warnings
  if (pattern.typicalBurnRate > 100) {
    adjustment -= 5;
  }

  return {
    warning: Math.max(50, Math.min(80, baseThresholds.warning + adjustment)),
    critical: Math.max(70, Math.min(90, baseThresholds.critical + adjustment)),
    danger: Math.max(85, Math.min(98, baseThresholds.danger + adjustment))
  };
}

export function getAdaptiveWarningLevel(percentage: number, thresholds: { warning: number; critical: number; danger: number }): WarningLevel {
  if (percentage >= thresholds.danger) return 'danger';
  if (percentage >= thresholds.critical) return 'critical';
  if (percentage >= thresholds.warning) return 'warning';
  return 'safe';
}

export function getSmartWarningMessage(
  percentage: number,
  pattern: UsagePattern,
  timeToLimit: string | null,
  isAdaptive: boolean = true
): string | null {
  if (percentage < 50) return null;
  
  const thresholds = getAdaptiveThresholds(pattern, 'day');
  const level = getAdaptiveWarningLevel(percentage, thresholds);
  
  if (level === 'safe') return null;
  
  const baseMessage = getAdvancedWarningMessage(percentage, timeToLimit, pattern.typicalBurnRate);
  
  if (!isAdaptive || !baseMessage) return baseMessage;
  
  // Add personalized insights
  let insight = "";
  
  if (pattern.consistencyScore > 0.8 && percentage > thresholds.critical) {
    insight = " You're usually more consistent - consider reviewing today's usage.";
  } else if (pattern.averageDailyUsage > 0 && percentage < pattern.averageDailyUsage / 1000000 * 100) {
    insight = " You're below your typical usage today.";
  } else if (percentage > 85 && pattern.typicalBurnRate > 0) {
    const projectedUsage = pattern.typicalBurnRate * 8 * 60; // 8 hours projection
    if (projectedUsage > 800000) { // 800K projection
      insight = " Your current rate suggests heavy usage ahead.";
    }
  }
  
  return baseMessage + insight;
}
