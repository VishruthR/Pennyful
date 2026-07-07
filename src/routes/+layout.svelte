<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
  import '../app.css';
    import { plaidApi } from '$lib/api/plaid';
  
  async function handleDeepLink(urls: string[]) {
    console.log("urls: ", urls);
    if (urls[0] == "pennyful://plaid-complete") {
      console.log("should generate an access token");
      try {
        let access_token = await plaidApi.generateAccessTokenFromHostedLink();
        console.log("access_token", access_token);
      } catch (err) {
        console.log(err);
      }
    }
  }

  onMount(() => {
    // cold start  
    getCurrent().then((urls) => urls && handleDeepLink(urls));

    // warm start
    const unlisten = onOpenUrl(handleDeepLink);
    return () => { unlisten.then((f) => f()); };
  })
</script>

<slot />

