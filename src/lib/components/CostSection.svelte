<script lang="ts">
  import type { UsageStats, UsageSummary, Period } from '../types';
  import { formatCost, getPeriodLabel, getPeriodCost } from '../utils';

  interface Props {
    usageStats: UsageStats;
    summary: UsageSummary | null;
    period: Period;
  }

  let { usageStats, summary, period }: Props = $props();
</script>

<div class="info-section">
  <div class="section-header">
    <div class="icon-title">
      <span class="section-title">Cost</span>
    </div>
  </div>
  <div class="status-line">
    <span class="status-text">Usage Tracking</span>
  </div>

  <div class="metrics">
    <div class="metric-row">
      <span class="label">Session:</span>
      {#if usageStats.active_session}
        <span class="value cost">{formatCost(usageStats.session_cost)}</span>
      {:else}
        <span class="value inactive-text">No cost</span>
      {/if}
    </div>
    <div class="metric-row">
      <span class="label">{getPeriodLabel(period)}:</span>
      <span class="value cost">{formatCost(getPeriodCost(period, usageStats, summary))}</span>
    </div>
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

  .value.cost {
    color: #10b981;
  }

  .value.inactive-text {
    color: rgba(255, 255, 255, 0.4);
    font-style: italic;
  }
</style>
