<script lang="ts">
  import Stepper from "$lib/components/Stepper.svelte";
  import AccountCard from "$lib/components/AccountCard.svelte";
  import type { Account, AccountsGetResponse, PlaidAccount, PlaidItem } from "$lib/types";
  import PlaidLink from "$lib/components/PlaidLink.svelte";
    import { plaidApi } from "$lib/api/plaid";
    import Button from "$lib/components/Button.svelte";
    import { goto } from "$app/navigation";
    import { PUBLIC_ITEM_ID } from "$env/static/public";

  // TODO: Handle errors

  // Step 1 state
  // let item_id: string = $state("");
  let item_id: string = $state(PUBLIC_ITEM_ID);

  // Step 2 state
  let alreadyAddedAccounts: (string | null)[] = $state([]);
  let accounts: AccountsGetResponse | null = $state(null);
  let selectedAccounts: number[] = $state([]);

  // Step 3 state
  let numTransactions: number | null = $state(null);

  const syncTransactions = async () => {
    numTransactions = await plaidApi.syncTransactions(item_id);
  }

  const getAccountSubname = (account: PlaidAccount, item: PlaidItem) => {
    if (account.official_name == account.name) {
      return item.institution_name ?? "Account";
    }

    return account.official_name ?? item.institution_name ?? "Account"
  }

  const steps = [
    {
      name: "Link an institution",
      content: step1Content,
      onNext: async () => {
        currentStep = 1;

        alreadyAddedAccounts = (await plaidApi.getAccountsOfItem(item_id)).map((a) => a.plaid_account_id);
        accounts = await plaidApi.getAccountsOfItemFromPlaid(item_id);
      },
    },
    {
      name: "Select bank accounts",
      content: step2Content,
      canProceed: () => selectedAccounts.length > 0,
      onNext: async () => {
        currentStep = 2;

        if (accounts === null) {
          return;
        }

        await plaidApi.addNewPlaidAccounts(
          accounts.accounts.filter((_, idx) => idx in selectedAccounts),
          item_id
        )
      },
      onBack: () => {
        currentStep = 0;
      },
    },
    {
      name: "Sync Transactions",
      content: step3Content,
      canProced: () => numTransactions !== null,
      onNext: async () => {
        goto("/")
      },
      onBack: () => {
        currentStep = 1;
      },
    },
  ];

  $inspect(accounts);
  $inspect(alreadyAddedAccounts);
  const accountsToSelect: PlaidAccount[] = $derived.by(() => {
     return accounts !== null 
      ? accounts.accounts.filter(a => !alreadyAddedAccounts.includes(a.account_id))
      : [];
  });
  const accountsAlreadyAdded: PlaidAccount[] = $derived.by(() => {
    return accounts !== null
      ? accounts.accounts.filter(a => alreadyAddedAccounts.includes(a.account_id))
      : [];
  })
</script>

{#snippet step1Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Link an institution</h2>
      <p class="paragraph step-description">The button below will use Plaid Link's interface, which will let you connect a financial institution thru Plaid and begin pulling transactions.</p>
      <p class="paragraph step-note">Note: If you just created your Plaid account, institutions that use OAuth may not be available immediately. It can take up to a day for an institution to grant your Plaid account OAuth access. If this is the case, please try again in a few hours.</p>
    </div>
    <PlaidLink bind:item_id={item_id} />
  </div>
{/snippet}

{#snippet step2Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2">Select bank accounts</h2>
      <p class="paragraph step-description">Select which accounts you want to add to Pennyful</p>
    </div>
   
    {#if accounts !== null && alreadyAddedAccounts !== null}
      {#if accountsToSelect.length > 0}
        <div class="accounts-grid">
          {#each accountsToSelect as account, idx}
            <AccountCard
              icon="mdi:bank"
              name={account.name}
              subname={getAccountSubname(account, (accounts as AccountsGetResponse).item)}
              accountType={account.type}
              balance={account.balances.current}
              selected={selectedAccounts.includes(idx)}
              onClick={() => {
                if (selectedAccounts.includes(idx)) {
                  selectedAccounts = selectedAccounts.filter((i) => i !== idx);
                } else {
                  selectedAccounts.push(idx)
                }
              }}
            />
          {/each}
        </div>
      {:else}
        <p class="paragraph step-description">No accounts found from this institution.</p>
      {/if}
      {#if accountsAlreadyAdded.length > 0}
        <div class="step-text-container">
          <h3 class="h3 step-subtitle">Already added</h3>
        </div>
        <div class="accounts-grid">
          {#each accountsAlreadyAdded as account}
            <AccountCard
              icon="mdi:bank"
              name={account.name}
              subname={getAccountSubname(account, (accounts as AccountsGetResponse).item)}
              accountType={account.type}
              balance={account.balances.current}
              disabled
              onClick={() => {}}
            />
          {/each}
        </div>
      {/if}
    {:else}
      <p>Loading...</p>
    {/if}
  </div>
{/snippet}

{#snippet step3Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Sync Transactions</h2>
      <p class="paragraph step-description">Sync transactions from the past 30 days with the button below.</p>
      <p class="paragraph step-description">For Plaid to sync transactions, it must get permission from the financial institution which can take up to 30 minutes. If you don't see any transactions synced, come back to Pennyful in a bit and try again.</p>
      <p class="paragraph step-description">Plaid pulls transaction data from its linked institutions between 1 to 4 times per day, so transactions displayed here may be up to a day behind.</p>
    </div>
    <Button onclick={syncTransactions}>Sync transactions</Button>
    {#if numTransactions !== null}
      <p class="paragraph transactions-num">Synced {numTransactions} transactions.</p>
    {/if}
  </div>
{/snippet}

<main class="page-container">
  <h1 class="h1 page-title">Batch Add</h1>
  <div class="stepper-container">
    <Stepper {steps} />
  </div>
</main>

<style>
  .page-container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 32px;
    background-color: var(--background-white);
  }

  .stepper-container {
    width: 100%;
    max-width: 800px;
  }

  .page-title {
    align-self: flex-start;
  }

  .step-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 32px;
    gap: 8px;
  }

  .step-text-container {
    width: 100%;
    margin-bottom: 24px;
  }

  .step-subtitle {
    margin-bottom: 0px;
  }

  .step-description {
    color: var(--grey-300);
  }

  .step-note {
    font-size: 12px;
    color: var(--grey-200);
  }

  .transactions-num {
    margin: 0px;
    font-size: 12px;
    color: var(--profit-green)
  }

  .accounts-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
    width: 100%;
  }
</style>
