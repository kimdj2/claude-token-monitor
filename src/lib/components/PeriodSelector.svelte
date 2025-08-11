<script lang="ts">
  import type { Period } from '../types';

  interface Props {
    period: Period;
    onPeriodChange: (period: Period) => void;
  }

  let { period, onPeriodChange }: Props = $props();

  const periodOptions: Period[] = ['day', 'week', 'month'];
</script>

<div class="period-selector">
  {#each periodOptions as p}
    <button
      class="period-btn" class:active={period === p}
      onclick={() => onPeriodChange(p)}
    >
      {p}
    </button>
  {/each}
</div>

<style>
  .period-selector {
    display: flex;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    padding: 4px;
    gap: 2px;
  }

  .period-btn {
    flex: 1;
    padding: 8px 12px;
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    text-transform: capitalize;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .period-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
  }

  .period-btn.active {
    background: rgba(255, 255, 255, 0.15);
    color: #ffffff;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }
</style>
