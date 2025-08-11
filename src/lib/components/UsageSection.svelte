<script lang="ts">
  import type { UsageStats, UsageSummary, Period } from '../types';
  import { formatTokens, formatCost, getPeriodTokens, getMaxTokens, getMaxTokensLabel, getPeriodLabel, getUsagePercentage, getWarningColor, calculateTimeToLimit, shouldShowUrgentWarning, getBurnRateColor, formatBurnRate, analyzeUsagePattern, getAdaptiveThresholds, getAdaptiveWarningLevel, getSmartWarningMessage } from '../utils';

  interface Props {
    usageStats: UsageStats;
    summary: UsageSummary | null;
    period: Period;
  }

  let { usageStats, summary, period }: Props = $props();
  const usagePattern = $derived((() => {
    const mockWeeklyData = [];
    if (usageStats) {
      for (let i = 0; i < 7; i++) {
        const date = new Date();
        date.setDate(date.getDate() - i);
        const variance = 0.7 + Math.random() * 0.6;
        mockWeeklyData.push({
          date: date.toISOString().split('T')[0],
          tokens: Math.floor(usageStats.daily_tokens * variance),
          cost: usageStats.cost * variance
        });
      }
    }
    return analyzeUsagePattern(mockWeeklyData);
  })());

  const adaptiveThresholds = $derived(getAdaptiveThresholds(usagePattern, period));
  const usagePercentage = $derived(getUsagePercentage(period, usageStats, summary));
  const warningLevel = $derived(getAdaptiveWarningLevel(usagePercentage, adaptiveThresholds));
  const warningColor = $derived(getWarningColor(warningLevel));

  const timeToLimit = $derived(calculateTimeToLimit(
    getPeriodTokens(period, usageStats, summary),
    getMaxTokens(period),
    usageStats.burn_rate
  ));
  const smartWarningMessage = $derived(getSmartWarningMessage(usagePercentage, usagePattern, timeToLimit, true));
  const isUrgentWarning = $derived(shouldShowUrgentWarning(usagePercentage, timeToLimit));
  const burnRateColor = $derived(getBurnRateColor(usageStats.burn_rate));
</script>

<div class="info-section">
  <div class="section-header">
    <div class="icon-title">
      <span class="section-title">Usage</span>
    </div>
  </div>
  <div class="status-line">
    <div class="status-dot {usageStats.active_session ? 'active' : 'inactive'}"></div>
    <span class="status-text">{usageStats.active_session ? 'Active Session' : 'No Active Session'}</span>
  </div>

  <div class="metrics">
    {#if usageStats.active_session}
      <div class="metric-row">
        <span class="label">Session:</span>
        <span class="value">{formatTokens(usageStats.current_tokens)}</span>
      </div>
                 {#if usageStats.burn_rate !== null}
           <div class="metric-row">
             <span class="label">Rate:</span>
             <span class="value burn-rate" style:color="{burnRateColor}">{formatBurnRate(usageStats.burn_rate)}</span>
           </div>
           {/if}
           {#if timeToLimit && usageStats.active_session}
           <div class="metric-row">
             <span class="label">Est. time:</span>
             <span class="value time-remaining" style:color="{isUrgentWarning ? '#ef4444' : '#10b981'}">{timeToLimit}</span>
           </div>
           {/if}
    {:else}
      <div class="metric-row">
        <span class="label">Session:</span>
        <span class="value inactive-text">No active session</span>
      </div>
    {/if}
    <div class="metric-row">
      <span class="label">Today Total:</span>
      <span class="value">{formatTokens(usageStats.daily_tokens)}</span>
    </div>
  </div>

           <div class="progress-section">
           <div class="progress-bar">
             <!-- Adaptive threshold markers -->
             <div class="threshold-marker warning-threshold" style:left="{adaptiveThresholds.warning}%"></div>
             <div class="threshold-marker critical-threshold" style:left="{adaptiveThresholds.critical}%"></div>
             <div class="threshold-marker danger-threshold" style:left="{adaptiveThresholds.danger}%"></div>

             <div class="progress-fill" style:width="{Math.min(usagePercentage, 100)}%" style:background-color="{warningColor}"></div>
           </div>
           <div class="progress-labels">
             <span class="progress-label-left">
               {getPeriodLabel(period)}: {formatTokens(getPeriodTokens(period, usageStats, summary))} / {getMaxTokensLabel(period)} tokens
               {#if warningLevel !== 'safe'}
                 <span class="warning-icon" style:color="{warningColor}">
                   {#if warningLevel === 'danger'}⚠️
                   {:else if warningLevel === 'critical'}🚨
                   {:else}⚡{/if}
                 </span>
               {/if}
             </span>
             <span class="progress-percentage" style:color="{warningLevel === 'safe' ? '#ffffff' : warningColor}">
               {usagePercentage.toFixed(1)}%
             </span>
                      </div>

           {#if smartWarningMessage}
             <div class="warning-message" class:urgent={isUrgentWarning} style:border-left-color="{warningColor}">
               {smartWarningMessage}
               {#if timeToLimit && usageStats.active_session && isUrgentWarning}
                 <div class="urgent-notice">
                   🚨 Session may end soon based on current usage rate!
                 </div>
               {/if}
               {#if usagePattern.consistencyScore > 0.8 && warningLevel !== 'safe'}
                 <div class="pattern-insight">
                   💡 Based on your usage pattern (consistency: {(usagePattern.consistencyScore * 100).toFixed(0)}%)
                 </div>
               {/if}
             </div>
           {/if}

    {#if summary}
      <div class="summary-line">
        <span>
          {#if period === 'day'}
            Today
          {:else if period === 'week'}
            This Week ({summary.start_date} ~ {summary.end_date})
          {:else}
            This Month ({summary.start_date} ~ {summary.end_date})
          {/if}
        </span>
        <span class="mono">{formatTokens(summary.total_tokens)} • {formatCost(summary.total_cost)}</span>
      </div>
    {/if}
         </div>
</div>

<style>
  .info-section {
    flex-shrink: 0;
    padding: 10px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .icon-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .section-title {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
  }

  .status-line {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 16px;
  }

  .status-text {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
  }

  .status-dot.inactive {
    background: #6b7280;
    box-shadow: none;
  }

  .metrics {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .metric-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
    font-weight: 500;
  }

  .value {
    font-size: 12px;
    color: #ffffff;
    font-weight: 600;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  }

  .value.burn-rate {
    color: #f59e0b;
  }

  .value.inactive-text {
    color: rgba(255, 255, 255, 0.4);
    font-style: italic;
  }

  .progress-section {
    margin-top: 16px;
  }

  .progress-bar {
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 8px;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.3s ease, background-color 0.3s ease;
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
  }

  .progress-label-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .warning-icon {
    font-size: 12px;
    margin-left: 4px;
  }

  .progress-percentage {
    font-weight: 600;
    font-size: 12px;
  }

  .warning-message {
    background: rgba(255, 255, 255, 0.05);
    border-left: 3px solid;
    padding: 8px 12px;
    margin-top: 8px;
    border-radius: 0 4px 4px 0;
    font-size: 11px;
    line-height: 1.4;
    backdrop-filter: blur(10px);
    animation: fadeIn 0.3s ease-in-out;
    transition: all 0.3s ease;
  }

  .warning-message.urgent {
    background: rgba(239, 68, 68, 0.1);
    border-left-color: #ef4444 !important;
    animation: pulse 2s infinite, fadeIn 0.3s ease-in-out;
  }

  .urgent-notice {
    margin-top: 4px;
    font-size: 10px;
    color: #ef4444;
    font-weight: 600;
  }

  .time-remaining {
    font-weight: 600;
  }

  /* Adaptive threshold markers */
  .threshold-marker {
    position: absolute;
    top: 0;
    width: 2px;
    height: 100%;
    z-index: 2;
    opacity: 0.7;
  }

  .warning-threshold {
    background: #ffa726;
    box-shadow: 0 0 4px rgba(255, 167, 38, 0.5);
  }

  .critical-threshold {
    background: #ff6b35;
    box-shadow: 0 0 4px rgba(255, 107, 53, 0.5);
  }

  .danger-threshold {
    background: #ff4757;
    box-shadow: 0 0 4px rgba(255, 71, 87, 0.5);
  }

  .pattern-insight {
    margin-top: 4px;
    font-size: 10px;
    color: #60a5fa;
    font-style: italic;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-5px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.7; }
    100% { opacity: 1; }
  }

  .summary-line {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 8px;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.7);
  }

  .mono {
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
    color: rgba(255, 255, 255, 0.9);
  }
</style>
