<script lang="ts">
  import Button from "$lib/components/Button.svelte";
</script>

<main class="container">
  <a href="/import/batch" class="batch-link">
    <Button>Batch Add Transactions</Button>
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
  }

  .batch-link {
    text-decoration: none;
  }
</style>
