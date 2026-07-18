<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import TransactionsTable from "$lib/components/TransactionsTable.svelte";
 </script>

<main class="container">
  <TransactionsTable />
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
</style>
