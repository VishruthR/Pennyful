<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
  import '../app.css';
    import { plaidApi } from '$lib/api/plaid';
  
  async function handleDeepLink(urls: string[]) {
    console.log("urls: ", urls);
    if (urls[0] == "pennyful://plaid-complete") {
      plaidApi.generateAccessTokenFromHostedLink();
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

