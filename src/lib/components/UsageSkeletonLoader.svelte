<script lang="ts">
  import SkeletonLoader from './SkeletonLoader.svelte';
</script>

<div class="info-section">
  <div class="section-header">
    <div class="icon-title">
      <span class="section-title">Usage</span>
    </div>
  </div>

  <div class="status-line">
    <SkeletonLoader width="8px" height="8px" borderRadius="50%" />
    <SkeletonLoader width="120px" height="12px" />
  </div>

  <div class="metrics">
    <div class="metric-row">
      <SkeletonLoader width="60px" height="12px" />
      <SkeletonLoader width="80px" height="12px" />
    </div>
    <div class="metric-row">
      <SkeletonLoader width="40px" height="12px" />
      <SkeletonLoader width="70px" height="12px" />
    </div>
    <div class="metric-row">
      <SkeletonLoader width="80px" height="12px" />
      <SkeletonLoader width="90px" height="12px" />
    </div>
  </div>

  <div class="progress-section">
    <div class="progress-bar">
      <SkeletonLoader width="100%" height="6px" borderRadius="3px" />
    </div>
    <div class="progress-labels">
      <SkeletonLoader width="180px" height="11px" />
      <SkeletonLoader width="40px" height="12px" />
    </div>

    <div class="summary-skeleton">
      <SkeletonLoader width="100%" height="16px" />
    </div>
  </div>
</div>

<style>
  .info-section {
    flex-shrink: 0;
    padding: 12px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
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

  .progress-labels {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .summary-skeleton {
    margin-top: 8px;
  }
</style>
