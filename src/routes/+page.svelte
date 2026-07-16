<script lang="ts">
  import Button from "$lib/components/Button.svelte";
    import IconPicker from "$lib/components/IconPicker.svelte";
 </script>

<main class="container">
  <div class="test">
    <IconPicker 
      hex={null}
      placeholder="Choose an icon"
      name="icon-picker"
    />
  </div>
  <a href="/onboarding" class="batch-link">
    <Button>Onboarding</Button>
  </a>
  <a href="/plaid/link" class="batch-link">
    <Button>Add Plaid Account</Button>
  </a>
</main>

<style>
  .test {
    width: 200px;
  }

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
