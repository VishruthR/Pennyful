<script lang="ts">
  import FileDrop from "$lib/components/FileDrop.svelte";
    import Flashcard from "$lib/components/Flashcard.svelte";

  function handleUpload(files: File[]) {
    console.log('Files uploaded:', files.map(f => f.name));
  }

  const transaction = {
    id: 1,
    name: "COSTCO WHSE #0008",
    amount: -190.10,
    date: new Date("2025-11-10"),
    account: {
      id: 1,
      name: "BOFA",
    },
    category: {
      id: 1,
      name: "Housing",
      color: "#A78BFA",
      icon: "mdi:home",
    },
  };
</script>

<main class="container">
  <h1 class="h2">FileDrop Component Demo</h1>

  <div class="flashcard-container">
    <Flashcard transaction={transaction} />
  </div>
  
  <div class="demo-section">
    <FileDrop
      onUpload={handleUpload}
      acceptedTypes={['.csv']}
      multiple={true}
    />
  </div>
</main>

<style>
  .container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 32px;
    padding: 32px;
  }

  .demo-section {
    width: 100%;
    max-width: 600px;
  }
  
  .flashcard-container {
    width: 100%;
    max-width: 400px;
  }
</style>
