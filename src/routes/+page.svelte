<script lang="ts">
  import Icon from "@iconify/svelte";
  import FileDrop from "$lib/components/FileDrop.svelte";
  import Dropdown from "$lib/components/Dropdown.svelte";

  function handleUpload(files: File[]) {
    console.log('Files uploaded:', files.map(f => f.name));
  }

  let selectedCategory = $state<string | null>(null);

  function handleCategorySelect(value: string) {
    selectedCategory = value;
    console.log('Selected category:', value);
  }
</script>

{#snippet housingOption()}
  <Icon icon="mdi:home" width={20} height={20} style="color: #9B7ED9;" />
  <span class="paragraph-bold">Housing</span>
{/snippet}

{#snippet transportationOption()}
  <Icon icon="mdi:car" width={20} height={20} style="color: #E57373;" />
  <span class="paragraph-bold">Transportation</span>
{/snippet}

{#snippet hobbiesOption()}
  <Icon icon="mdi:basketball" width={20} height={20} style="color: #64B5F6;" />
  <span class="paragraph-bold">Hobbies</span>
{/snippet}

{#snippet subscriptionsOption()}
  <Icon icon="mdi:calendar-month" width={20} height={20} style="color: #E57373;" />
  <span class="paragraph-bold">Subscriptions</span>
{/snippet}

{#snippet healthcareOption()}
  <Icon icon="mdi:heart-plus" width={20} height={20} style="color: #9575CD;" />
  <span class="paragraph-bold">Healthcare</span>
{/snippet}

<main class="container">
  <h1 class="h2">Component Demos</h1>

  <section class="demo-section">
    <h2 class="h3">Dropdown Component</h2>
    <div class="dropdown-demo">
      <Dropdown
        options={[
          { value: 'housing', content: housingOption },
          { value: 'transportation', content: transportationOption },
          { value: 'hobbies', content: hobbiesOption },
          { value: 'subscriptions', content: subscriptionsOption },
          { value: 'healthcare', content: healthcareOption },
        ]}
        onSelect={handleCategorySelect}
        selectedValue={selectedCategory}
        placeholder="Select..."
      />
      {#if selectedCategory}
        <p class="paragraph">Selected: <strong>{selectedCategory}</strong></p>
      {/if}
    </div>
  </section>
  
  <section class="demo-section">
    <h2 class="h3">FileDrop Component</h2>
    <FileDrop
      onUpload={handleUpload}
      acceptedTypes={['.csv']}
      multiple={true}
    />
  </section>
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
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .dropdown-demo {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
</style>
