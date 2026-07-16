<script lang="ts">
  import '../app.css';
  import { page } from "$app/state";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";

  let { children } = $props();

  // Route -> title shown in the top bar.
  const PAGE_TITLES: Record<string, string> = {
    "/": "Home",
    "/plaid/link": "Add Accounts",
    "/categories": "Categories",
    "/settings": "Settings",
    "/settings/plaid": "Plaid Credentials",
  };

  // Full-screen flows that should render without the app chrome.
  const CHROMELESS_ROUTES = ["/onboarding"];

  const showChrome = $derived(
    !CHROMELESS_ROUTES.some((prefix) => page.url.pathname.startsWith(prefix))
  );
  const title = $derived(PAGE_TITLES[page.url.pathname] ?? "");
</script>

{#if showChrome}
  <div class="app-shell">
    <Sidebar />
    <div class="main-area">
      <TitleBar {title} />
      <div class="page-content">
        {@render children()}
      </div>
    </div>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .app-shell {
    display: flex;
    height: 100vh;
  }

  .main-area {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .page-content {
    flex: 1;
    overflow: auto;
  }
</style>
