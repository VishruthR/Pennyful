<script lang="ts">
  import { goto } from "$app/navigation";
  import Stepper from "$lib/components/Stepper.svelte";
  import BankAccountCard from "$lib/components/BankAccountCard.svelte";
  import FileDrop from "$lib/components/FileDrop.svelte";
  import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
  import type { TransactionImport } from "$lib/types";
  import { importTransactions } from "$lib/api/importers";
  import { accountsApi } from "$lib/api/accounts";

  // TODO: Handle errors
  const bankAccounts = await accountsApi.getAllAccounts();

  let currentStep = $state(0);

  // Step 1 state
  let selectedAccountId = $state<number | null>(null);

  // Step 2 state
  let selectedFilePath = $state<string | null>(null);
  let importedTransactions = $state<TransactionImport[]>([]);

  // Step 3 state
  let acceptedTransactions = $state<TransactionImport[]>([]);

  function handleTransactionDiscard(transaction: TransactionImport) {
    console.log("Discarded:", transaction.name);
  }

  function handleTransactionAccept(transaction: TransactionImport) {
    console.log("Accepted:", transaction.name);
    acceptedTransactions = [...acceptedTransactions, transaction];
  }

  function handleReviewComplete() {
    console.log("Review complete!");
    console.log("Accepted transactions:", acceptedTransactions);
    goto("/");
  }

  const steps = [
    {
      name: "Select bank account",
      content: step1Content,
      canProceed: () => selectedAccountId != null,
      onNext: () => {
        currentStep = 1;
      },
    },
    {
      name: "Upload File",
      content: step2Content,
      canProceed: () => selectedFilePath != null,
      onNext: async () => {
        currentStep = 2;
        // TODO: Handle errors (no file selected or transactions import fails)
        if (!selectedFilePath || !selectedAccountId) {
          return;
        }
        importedTransactions = await importTransactions(
          selectedFilePath, 
          bankAccounts.find(account => account.id === selectedAccountId)?.bank_name ?? "", 
          selectedAccountId
        );
      },
      onBack: () => {
        currentStep = 0;
      },
    },
    {
      name: "Review",
      content: step3Content,
      hideNextButton: true,
      onBack: () => {
        currentStep = 1;
      },
    },
  ];
</script>

{#snippet step1Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Select bank account</h2>
      <p class="paragraph step-description">Select the bank account which you want to import transactions for</p>
    </div>
    
    <div class="accounts-grid">
      {#each bankAccounts as account}
        <BankAccountCard
          icon="mdi:bank"
          name={account.name}
          provider={account.bank_name}
          accountType={account.account_type}
          balance={account.current_balance}
          selected={selectedAccountId === account.id}
          onClick={() => {
            selectedAccountId = account.id;
          }}
        />
      {/each}
    </div>
  </div>
{/snippet}

{#snippet step2Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Upload File</h2>
      <p class="paragraph step-description">Upload the file containing your transaction data</p>
    </div>
    
    <div class="file-drop-container">
      <FileDrop
        acceptedTypes={[".csv"]}
        filterName="CSV Files"
        onSelect={(paths) => {
          selectedFilePath = paths[0] ?? null;
        }}
        multiple={true}
      />
    </div>
  </div>
{/snippet}

{#snippet step3Content()}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Review</h2>
      <p class="paragraph step-description">Take a moment to review the transactions you've imported. If any fields are incorrect or missing, click on them to manually edit.</p>
    </div>
    
    <div class="review-container">
      <FlashcardDeck
        transactions={importedTransactions}
        discardText="Delete"
        acceptText="Submit"
        onDiscard={handleTransactionDiscard}
        onAccept={handleTransactionAccept}
        onComplete={handleReviewComplete}
      />
    </div>
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
    max-width: 700px;
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
    margin-bottom: 36px;
  }

  .step-title {
    margin: 0px;
  }

  .step-description {
    margin: 0px;
    color: var(--grey-300);
  }

  /* Step 1: Account selection */
  .accounts-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
    width: 100%;
  }

  /* Step 2: File upload */
  .file-drop-container {
    width: 100%;
    min-height: 200px;
    display: flex;
    align-items: center;
  }

  /* Step 3: Review */
  .review-container {
    display: flex;
    justify-content: center;
    width: 100%;
    max-width: 450px;
    margin-top: 16px;
  }
</style>
