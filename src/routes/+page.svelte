<script lang="ts">
  import FileDrop from "$lib/components/FileDrop.svelte";

  function handleUpload(files: File[]) {
    console.log('Files uploaded:', files.map(f => f.name));
  }
</script>

<main class="container">
  <h1 class="h2">FileDrop Component Demo</h1>
  
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
</style>
