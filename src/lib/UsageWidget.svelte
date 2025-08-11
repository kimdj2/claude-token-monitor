<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import type { UsageStats, UsageSummary, Period } from './types';
  import PeriodSelector from './components/PeriodSelector.svelte';
  import UsageSection from './components/UsageSection.svelte';
  import CostSection from './components/CostSection.svelte';
  import ModelSection from './components/ModelSection.svelte';
  import LoadingState from './components/LoadingState.svelte';
  import ErrorState from './components/ErrorState.svelte';
  import UsageSkeletonLoader from './components/UsageSkeletonLoader.svelte';

  let period: Period = $state('day');
  let summary = $state<UsageSummary | null>(null);
  let usageStats = $state<UsageStats | null>(null);
  let loading = $state(true);
  let summaryLoading = $state(false);
  let error = $state<string | null>(null);
  let monitorElement: HTMLElement | undefined;

  const refreshMs = 5000;
  const cacheTimeout = 3000;
  let lastFetchTime = 0;
  let lastSummaryFetchTime = 0;
  let lastPeriod = '';
  let outsideClickListenerActive = false;

  async function fetchUsage() {
    const now = Date.now();
    if (usageStats && (now - lastFetchTime) < cacheTimeout) {
      return;
    }
    try {
      loading = true;
      error = null;
      usageStats = await invoke<UsageStats>("get_claude_usage");
      lastFetchTime = now;
    } catch (err) {
      error = err as string;
    } finally {
      loading = false;
    }
  }

  async function fetchUsageSummary(showLoading = false) {
    const now = Date.now();
    const periodChanged = lastPeriod !== period;
    if (periodChanged) {
      lastPeriod = period;
    }
    if (!periodChanged && summary && (now - lastSummaryFetchTime) < cacheTimeout) {
      return;
    }

    if (showLoading) {
      summaryLoading = true;
    }

    try {
      const s = await invoke<UsageSummary>('get_usage_summary', { period });
      summary = s;
      lastSummaryFetchTime = now;
    } catch (err) {
      console.error('Failed to fetch summary', err);
    } finally {
      if (showLoading) {
        summaryLoading = false;
      }
    }
  }

  const onKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      removeOutsideClickListener();
      invoke("hide_main_window");
    }
  };

  const handleOutsideClick = (e: MouseEvent) => {
    const target = e.target as Element;
    const monitor = document.querySelector('.monitor');
    if (monitor && !monitor.contains(target)) {
      removeOutsideClickListener();
      invoke("hide_main_window");
    }
  };

  const addOutsideClickListener = () => {
    if (!outsideClickListenerActive) {
      setTimeout(() => {
        document.addEventListener("click", handleOutsideClick, true);
        outsideClickListenerActive = true;
      }, 200);
    }
  };

  const removeOutsideClickListener = () => {
    if (outsideClickListenerActive) {
      document.removeEventListener("click", handleOutsideClick, true);
      outsideClickListenerActive = false;
    }
  };

  const handleWindowFocus = () => {
    addOutsideClickListener();
  };

  const handleWindowBlur = () => {
    removeOutsideClickListener();
  };

  const handlePeriodChange = async (newPeriod: Period) => {
    if (period !== newPeriod) {
      period = newPeriod;
      await fetchUsageSummary(true);
    }
  };

  $effect(() => {
    if (period) {
      fetchUsageSummary();
    }
  });

  onMount(async () => {
    await Promise.all([fetchUsage(), fetchUsageSummary()]);

    const interval = setInterval(fetchUsage, refreshMs);

    const summaryInterval = setInterval(fetchUsageSummary, refreshMs * 3);
    const unlisten = await listen('refresh-usage', () => {
      fetchUsage();
    });

    window.addEventListener("keydown", onKey);
    window.addEventListener("focus", handleWindowFocus);
    window.addEventListener("blur", handleWindowBlur);
    addOutsideClickListener();

    return () => {
      clearInterval(interval);
      if (unlisten) unlisten();
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("focus", handleWindowFocus);
      window.removeEventListener("blur", handleWindowBlur);
      removeOutsideClickListener();
      clearInterval(summaryInterval);
    };
  });
</script>

<div class="monitor" bind:this={monitorElement}>
  {#if loading && !usageStats}
    <LoadingState />
  {:else if error}
    <ErrorState error={error} onRetry={fetchUsage} />
  {:else if usageStats}
    <div class="header-section">
      <PeriodSelector period={period} onPeriodChange={handlePeriodChange} />
    </div>
    {#if summaryLoading}
      <UsageSkeletonLoader />
    {:else}
      <UsageSection usageStats={usageStats} summary={summary} period={period} />
    {/if}
    <CostSection usageStats={usageStats} summary={summary} period={period} />
    <ModelSection usageStats={usageStats} />
  {/if}
</div>

<style>
  .monitor {
    background: linear-gradient(145deg, rgba(28, 32, 44, 0.98), rgba(20, 24, 34, 0.98));
    backdrop-filter: blur(40px);
    border-radius: 12px;
    color: #ffffff;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
    width: 100vw;
    height: 100vh;
    max-height: 580px;
    margin: 0;
    padding: 0;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.15);
    box-shadow:
      0 16px 48px rgba(0, 0, 0, 0.8),
      0 4px 16px rgba(0, 0, 0, 0.4),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    flex-direction: column;
    border-radius: 12px;
    -webkit-border-radius: 12px;
    -moz-border-radius: 12px;
    -webkit-transform: translateZ(0);
    transform: translateZ(0);
    overflow: hidden;
  }

  .monitor::-webkit-scrollbar {
    display: none;
  }

  .monitor {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  .monitor::before {
    content: '';
    position: absolute;
    top: -8px;
    right: 20px;
    width: 16px;
    height: 16px;
    background: linear-gradient(145deg, rgba(28, 32, 44, 0.98), rgba(20, 24, 34, 0.98));
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-bottom: none;
    border-right: none;
    transform: rotate(45deg);
    z-index: -1;
  }

  .header-section {
    flex-shrink: 0;
    padding: 10px 20px 6px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }
</style>