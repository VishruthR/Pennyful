<script lang="ts">
    import { transactionsApi } from "$lib/api/transactions";
  import Button from "$lib/components/Button.svelte";

  const handleClick = async () => {
    console.log(await transactionsApi.getPaginatedSortedTransactions(
      2,
      10,
      "amount",
      "Desc"
    ))
  }
 </script>

<main class="container">
  <Button onclick={handleClick}>Test get transactions </Button>
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
