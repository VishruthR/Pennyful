<script lang="ts">
  import Stepper, { type StepContext } from "$lib/components/Stepper.svelte";

  function handleComplete(data: Record<number, unknown>) {
    console.log('Stepper completed with data:', data);
    alert(`Completed!\n\nCollected data:\n${JSON.stringify(data, null, 2)}`);
  }
</script>

<main class="container">
  <h1 class="h2">Stepper Component Demo</h1>
  
  <Stepper
    steps={[
      { name: 'Select bank account', content: step1Content,
      canProceed: (data) => {
        const stepData = data as { bankId: string } | undefined;
        return !!stepData?.bankId;
      },
      },
      { name: 'Choose Import Option', content: step2Content },
      { name: 'Upload File', content: step3Content }
    ]}
    onComplete={handleComplete}
  />
</main>

{#snippet step1Content(ctx: StepContext)}
  {@const savedData = ctx.getData() as { bankId: string } | undefined}
  <div class="step-page">
    <h2 class="h2">Select your bank account</h2>
    <p class="paragraph">
      Choose the bank account you want to import transactions from. 
      We support all major banks including Bank of America, Wells Fargo, 
      Chase, and American Express.
    </p>
    <div class="input-group">
      <label class="paragraph-bold" for="bank-select">Bank Account</label>
      <select
        id="bank-select"
        class="select-input"
        value={savedData?.bankId ?? ''}
        onchange={(e) => ctx.setData({ bankId: e.currentTarget.value })}
      >
        <option value="">Select a bank...</option>
        <option value="bofa">Bank of America</option>
        <option value="wells">Wells Fargo</option>
        <option value="chase">Chase</option>
        <option value="amex">American Express</option>
      </select>
    </div>
  </div>
{/snippet}

{#snippet step2Content(ctx: StepContext)}
  {@const savedData = ctx.getData() as { importType: string } | undefined}
  <div class="step-page">
    <h2 class="h2">Choose import option</h2>
    <p class="paragraph">
      Select how you want to import your transaction data.
    </p>
    <div class="radio-group">
      {#each [
        { value: 'csv', label: 'CSV File' },
        { value: 'json', label: 'JSON Data' },
        { value: 'api', label: 'Connect via API' }
      ] as option}
        <label class="radio-label">
          <input
            type="radio"
            name="importType"
            value={option.value}
            checked={savedData?.importType === option.value}
            onchange={() => ctx.setData({ importType: option.value })}
          />
          <span class="paragraph">{option.label}</span>
        </label>
      {/each}
    </div>
  </div>
{/snippet}

{#snippet step3Content(ctx: StepContext)}
  {@const savedData = ctx.getData() as { fileName: string } | undefined}
  <div class="step-page">
    <h2 class="h2">Upload your file</h2>
    <p class="paragraph">
      Enter the name of your transaction file.
    </p>
    <div class="input-group">
      <label class="paragraph-bold" for="file-name">File Name</label>
      <input
        id="file-name"
        type="text"
        class="text-input"
        placeholder="transactions.csv"
        value={savedData?.fileName ?? ''}
        oninput={(e) => ctx.setData({ fileName: e.currentTarget.value })}
      />
    </div>
  </div>
{/snippet}

<style>
  .container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    align-items: center;
    gap: 32px;
    padding: 32px;
  }

  .step-page {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .step-page h2 {
    margin: 0;
  }

  .step-page p {
    margin: 0;
    color: var(--grey-300);
    max-width: 600px;
    line-height: 1.6;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 8px;
  }

  .input-group label {
    color: var(--grey-500);
  }

  .select-input,
  .text-input {
    padding: 12px 16px;
    border: 2px solid var(--grey-100);
    border-radius: 8px;
    font-size: 16px;
    font-family: var(--font-family);
    color: var(--grey-500);
    background-color: var(--pure-white);
    max-width: 300px;
  }

  .select-input:focus,
  .text-input:focus {
    outline: none;
    border-color: var(--grey-300);
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 8px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
  }

  .radio-label input[type="radio"] {
    width: 20px;
    height: 20px;
    accent-color: var(--grey-500);
  }
</style>
