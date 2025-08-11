<script lang="ts">
  import type { UsageStats } from '../types';

  interface Props {
    usageStats: UsageStats;
  }

  let { usageStats }: Props = $props();
</script>

<div class="info-section">
  <div class="section-header">
    <div class="icon-title">
      <span class="section-title">Model</span>
    </div>
  </div>
  <div class="status-line">
    <span class="status-text">Currently Active</span>
  </div>
  <div class="model-info">
    <div class="model-name">{usageStats.model || 'No model detected'}</div>
  </div>
</div>

<style>
  .info-section {
    flex-shrink: 0;
    padding: 10px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .info-section:last-child {
    border-bottom: none;
    padding-bottom: 12px;
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
    margin-bottom: 12px;
  }

  .status-text {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
  }

  .model-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .model-name {
    font-size: 13px;
    font-weight: 600;
    color: #ffffff;
  }
</style>
