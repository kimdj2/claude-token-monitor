<script lang="ts">
  interface Props {
    error: string;
    onRetry: () => void;
  }

  let { error, onRetry }: Props = $props();
</script>

<div class="error">
  <p>Error: {error}</p>
  <button onclick={onRetry}>Retry</button>
</div>

<style>
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 16px;
  }

  .error p {
    font-size: 14px;
    color: #ef4444;
    margin: 0;
    text-align: center;
  }

  .error button {
    padding: 8px 16px;
    background: #10b981;
    color: #ffffff;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .error button:hover {
    background: #059669;
  }
</style>
