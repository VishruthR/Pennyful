<script lang="ts">
  import { goto } from "$app/navigation";
  import Stepper from "$lib/components/Stepper.svelte";
  import type { StepContext } from "$lib/components/Stepper.svelte";
  import BankAccountCard from "$lib/components/BankAccountCard.svelte";
  import FileDrop from "$lib/components/FileDrop.svelte";
  import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
  import type { FullTransactionInfo } from "$lib/types";

  // Mock data for bank accounts
  const mockAccounts = [
    { id: 1, icon: "mdi:bank", name: "BoFA Account", provider: "Bank of America", accountType: "Checking", balance: 1900.17 },
    { id: 2, icon: "mdi:bank", name: "BoFA Account", provider: "Bank of America", accountType: "Checking", balance: 1900.17 },
    { id: 3, icon: "mdi:bank", name: "BoFA Account", provider: "Bank of America", accountType: "Checking", balance: 1900.17 },
    { id: 4, icon: "mdi:bank", name: "BoFA Account", provider: "Bank of America", accountType: "Checking", balance: 1900.17 },
  ];

  // Mock transactions for review step
  const mockTransactions: FullTransactionInfo[] = [
    {
      id: 1,
      name: "CSTCO 11/28 KIRKLAND SIGNATURE BLAH BLAH BLAH BLAH BLAH",
      amount: -190.10,
      date: new Date("2025-11-10"),
      account: { id: 1, name: "BOFA" },
      category: { id: 1, name: "Housing", color: "#A78BFA", icon: "mdi:home" },
    },
    {
      id: 2,
      name: "AMAZON PRIME MEMBERSHIP",
      amount: -14.99,
      date: new Date("2025-11-08"),
      account: { id: 2, name: "AMEX" },
      category: { id: 2, name: "Subscriptions", color: "#60A5FA", icon: "mdi:credit-card" },
    },
    {
      id: 3,
      name: "PAYROLL DEPOSIT",
      amount: 3500.00,
      date: new Date("2025-11-01"),
      account: { id: 1, name: "BOFA" },
      category: { id: 3, name: "Income", color: "#4ADE80", icon: "mdi:cash" },
    },
  ];

  interface Step1Data {
    selectedAccountId: number | null;
  }

  interface Step2Data {
    file: File;
  }

  // Step 1 state
  // TODO: Feels like this state is redundant with internal state of stepper
  let selectedAccountId = $state<number | null>(null);

  // Step 2 state (Upload File)
  let uploadedFile = $state<File>();

  // Step 3 state (Review)
  let acceptedTransactions = $state<FullTransactionInfo[]>([]);

  function handleComplete(data: Record<number, unknown>) {
    console.log("Batch upload complete!", data);
    console.log("Accepted transactions:", acceptedTransactions);
    goto("/");
  }

  function handleTransactionDiscard(transaction: FullTransactionInfo) {
    console.log("Discarded:", transaction.name);
  }

  function handleTransactionAccept(transaction: FullTransactionInfo) {
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
      canProceed: (data: unknown) => {
        const stepData = data as Step1Data | undefined;
        return stepData?.selectedAccountId != null;
      },
    },
    {
      name: "Upload File",
      content: step2Content,
      canProceed: (data: unknown) => {
        const stepData = data as Step2Data | undefined;
        return stepData?.file != null;
      },
    },
    {
      name: "Review",
      content: step3Content,
      hideNextButton: true,
    },
  ];
</script>

{#snippet step1Content(ctx: StepContext)}
  {@const currentData = ctx.getData() as Step1Data | undefined}
  {@const currentSelection = currentData?.selectedAccountId ?? selectedAccountId}
  
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Select bank account</h2>
      <p class="paragraph step-description">Select the bank account which you want to import transactions for</p>
    </div>
    
    <div class="accounts-grid">
      {#each mockAccounts as account}
        <BankAccountCard
          icon={account.icon}
          name={account.name}
          provider={account.provider}
          accountType={account.accountType}
          balance={account.balance}
          selected={currentSelection === account.id}
          onClick={() => {
            selectedAccountId = account.id;
            ctx.setData({ selectedAccountId: account.id } satisfies Step1Data);
          }}
        />
      {/each}
    </div>
  </div>
{/snippet}

{#snippet step2Content(ctx: StepContext)}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Upload File</h2>
      <p class="paragraph step-description">Upload the file containing your transaction data</p>
    </div>
    
    <div class="file-drop-container">
      <FileDrop
        acceptedTypes={[".csv"]}
        onUpload={(files) => {
          uploadedFile = files[0];
          ctx.setData({ file: files[0] } satisfies Step2Data);
        }}
        multiple={false}
      />
    </div>
  </div>
{/snippet}

{#snippet step3Content(ctx: StepContext)}
  <div class="step-container">
    <div class="step-text-container">
      <h2 class="h2 step-title">Review</h2>
      <p class="paragraph step-description">Take a moment to review the transactions you've imported. If any fields are incorrect or missing, click on them to manually edit.</p>
    </div>
    
    <div class="review-container">
      <FlashcardDeck
        transactions={mockTransactions}
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
    <Stepper {steps} onComplete={handleComplete} />
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
    margin-top: 32px;
    gap: 8px;
  }

  .step-text-container {
    margin-bottom: 48px;
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
    height: 100%;
    min-height: 200px;
    display: flex;
    align-items: center;
  }

  /* Step 3: Review */
  .review-container {
    display: flex;
    justify-content: center;
    margin-top: 16px;
  }
</style>
