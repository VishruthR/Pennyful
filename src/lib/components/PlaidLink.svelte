<!-- @component
  A button that renders the full Plaid Link flow
  It will accept a callback which accepts the generated access token
-->

<script lang="ts">
  import { plaidApi } from "$lib/api/plaid";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Button from "./Button.svelte";

  let error = $state<string | null>(null);
  let loading = $state(false);
  let phone_number = $state("");

  async function handleLink() {
    loading = true;
    error = null;

    try {
      let hosted_link_url = await plaidApi.generateLinkToken(phone_number);
      await openUrl(hosted_link_url);
    } catch (err) {
      console.log(err);
    } finally {
      loading = false;
    }
  }

  async function handleRedirect() {
    try {
      await plaidApi.generateAccessTokenFromHostedLink();
    } catch (err) {
      console.log(err);
    }
  }
</script>

<div>
  <input bind:value={phone_number} />
  <Button onclick={handleLink}>
    {loading ? "Loading…" : "Link an Institution"}
  </Button>
  <Button onclick={handleRedirect}>Done with flow</Button>
  {#if error}
    <p class="error">{error}</p>
  {/if}
</div>

<style>
  
</style>
