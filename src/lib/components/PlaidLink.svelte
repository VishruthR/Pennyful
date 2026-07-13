<!-- @component
  A button that prompts the full Plaid Link flow
  In dev mode, it will include a button to get an access_token without requiring deep links
-->

<script lang="ts">
  import { plaidApi } from "$lib/api/plaid";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Button from "$lib/components/Button.svelte";

  let { item_id = $bindable() } = $props();

  let error = $state<string | null>(null);
  let loading = $state(false);

  async function handleLink() {
    loading = true;
    error = null;

    try {
      let hosted_link_url = await plaidApi.generateLinkToken();
      await openUrl(hosted_link_url);
    } catch (err) {
      error = err as string;
      console.log(error);
    } finally {
      loading = false;
    }
  }

  async function handleRedirect() {
    loading = true;
    error = null;

    try {
      item_id = await plaidApi.generateAccessTokenFromHostedLink();
    } catch (err) {
      error = err as string
      console.log(error);
    } finally {
      loading = false;
    }
  }
</script>

<div class="container">
  <Button onclick={handleLink}>
    {loading ? "Loading…" : "Link thru Plaid"}
  </Button>
  {#if import.meta.env.DEV}
    <!-- Dev-only: in `tauri dev` the pennyful:// deep link can't reach us, so
         finish the Hosted Link flow by hand. Prod uses the deep link. -->
    <button class="link-btn" onclick={handleRedirect}>Done with flow (dev)</button>
  {/if}
  {#if error}
    <p class="error">{error}</p>
  {/if}
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .link-btn {
    border: none;
    font-size: 12px;
    background-color: transparent;
    text-decoration: underline;
    color: var(--grey-200);
  }

  .link-btn:hover {
    cursor: pointer;
    color: var(--grey-400);
  }
</style>
