<script lang="ts">
    import { plaidApi } from "$lib/api/plaid";
  import Button from "$lib/components/Button.svelte";
  import PlaidLink from "$lib/components/PlaidLink.svelte";

  const syncTransactions = async () => {
    console.log(await plaidApi.fetchItemAndAccounts("AzvX08j5jbTapxEyxwQvcdVnBygpgafNjn5Ke"))
    console.log(await plaidApi.syncTransactions("AzvX08j5jbTapxEyxwQvcdVnBygpgafNjn5Ke"))
  };
</script>

<main class="container">
  <a href="/import/batch" class="batch-link">
    <Button>Batch Add Transactions</Button>
  </a>
  <PlaidLink />
  <Button onclick={syncTransactions}>Sync Transactions</Button>
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
