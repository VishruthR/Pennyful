<script lang="ts">
  import Button from "$lib/components/Button.svelte";
 </script>

<main class="container">
  <a href="/onboarding" class="batch-link">
    <Button>Onboarding</Button>
  </a>
  <a href="/plaid/link" class="batch-link">
    <Button>Add Plaid Account</Button>
  </a>
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
    gap: 48px;
  }

  .batch-link {
    text-decoration: none;
  }
</style>
