<script lang="ts">
  import Stepper from "$lib/components/Stepper.svelte";

  function handleComplete() {
    console.log('Stepper completed!');
    alert('All steps completed!');
  }
</script>

<main class="container">
  <h1 class="h2">Stepper Component Demo</h1>
  
  <Stepper
    steps={[
      { name: 'Select bank account', content: step1Content },
      { name: 'Choose Import Option', content: step2Content },
      { name: 'Upload File', content: step3Content }
    ]}
    onComplete={handleComplete}
  />
</main>

{#snippet step1Content()}
  <div class="step-page">
    <h2 class="h2">Select your bank account</h2>
    <p class="paragraph">
      Choose the bank account you want to import transactions from. 
      We support all major banks including Bank of America, Wells Fargo, 
      Chase, and American Express.
    </p>
  </div>
{/snippet}

{#snippet step2Content()}
  <div class="step-page">
    <h2 class="h2">Choose import option</h2>
    <p class="paragraph">
      Select how you want to import your transaction data. You can upload 
      a CSV file, connect via API, or paste JSON data directly.
    </p>
  </div>
{/snippet}

{#snippet step3Content()}
  <div class="step-page">
    <h2 class="h2">Upload your file</h2>
    <p class="paragraph">
      Drag and drop your transaction file here, or click to browse. 
      We accept CSV files exported from your bank's website.
    </p>
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
</style>
