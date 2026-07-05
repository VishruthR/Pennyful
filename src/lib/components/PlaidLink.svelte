<!-- @component
  A button that renders the full Plaid Link flow
  It will accept a callback which accepts the generated access token
-->

<script lang="ts">
  import { plaidApi } from "$lib/api/plaid";
  import Button from "./Button.svelte";

  let error = $state<string | null>(null);
  let loading = $state(false);

  async function handleLink() {
    if (typeof Plaid === "undefined") {
      error = "Plaid SDK not loaded yet";
      return;
    }

    loading = true;
    error = null;

    try {
      const link_token = await plaidApi.generateLinkToken();

      const handler = Plaid.create({
        token: link_token,
        onSuccess: (public_token, metadata) => {
          console.log("public_token", public_token, metadata);
          plaidApi.generateAccessToken(public_token);
        },
        onLoad: () => {},
        onExit: (err, metadata) => {
          if (err) console.error("Plaid exit", err, metadata);
        },
        onEvent: (eventName, metadata) => {
          console.log("Plaid event", eventName, metadata);
        },
      });

      handler.open();
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  }
</script>

<div>
  <Button onclick={handleLink} disabled={loading}>
    {loading ? "Loading…" : "Link an Institution"}
  </Button>
  {#if error}
    <p class="error">{error}</p>
  {/if}
</div>

<style>
  
</style>
