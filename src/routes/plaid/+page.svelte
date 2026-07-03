<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import { plaidApi } from "$lib/api/plaid";

  let status = $state<string | null>(null);

  async function handleConnect() {
    status = "Connecting…";
    try {
      const accessToken = await plaidApi.connectToPlaid();
      status = `Connected. Number of Transactions: ${accessToken}`;
    } catch (error) {
      console.error("Failed to connect to Plaid:", error);
      status = `Error: ${error}`;
    }
  }
</script>

<main class="container">
  <Button onclick={handleConnect}>Connect Link</Button>
  {#if status}
    <p class="status">{status}</p>
  {/if}
</main>

<style>
  .container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px;
    gap: 24px;
  }

  .status {
    color: var(--grey-500);
  }
</style>
