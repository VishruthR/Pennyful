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
    // if (typeof Plaid === "undefined") {
    //   error = "Plaid SDK not loaded yet";
    //   return;
    // }

    loading = true;
    error = null;

    try {
      console.log("here for sure");
      let hosted_link_url = await plaidApi.generateLinkToken(phone_number);
      console.log(hosted_link_url);
      await openUrl(hosted_link_url);

      // const handler = Plaid.create({
      //   token: link_token,
      //   onSuccess: (public_token, metadata) => {
      //     // plaidApi.generateAccessToken(public_token);
      //     console.log("temp");
      //   },
      //   onLoad: () => {},
      //   onExit: (err, metadata) => {
      //     if (err) console.error("Plaid exit", err, metadata);
      //   },
      //   onEvent: (eventName, metadata) => {
      //     console.log("Plaid event", eventName, metadata);
      //   },
      // });

      // handler.open();
    } catch (err) {
      console.log(err);
    } finally {
      loading = false;
    }
  }

  async function handleRedirect() {
    try {
      plaidApi.generateAccessTokenFromHostedLink();
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
