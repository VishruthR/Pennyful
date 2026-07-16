<script lang="ts">
    import Icon from "@iconify/svelte";
</script>

<main class="page">
  <h2 class="h2">Settings</h2>
  <p class="paragraph">Manage your preferences and account settings.</p>

  <nav class="settings-list">
    <a class="settings-link paragraph" href="/settings/plaid">
      <span>Plaid credentials</span>
      <Icon icon="material-symbols:chevron-right" width={24} />
    </a>
  </nav>
</main>

<style>
  .page {
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .settings-list {
    display: flex;
    flex-direction: column;
    margin-top: 24px;
    border: 1.5px solid var(--grey-100);
    border-radius: 10px;
    overflow: hidden;
    max-width: 540px;
  }

  .settings-link {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    color: var(--grey-500);
    text-decoration: none;
    transition: background-color 0.15s ease;
  }

  .settings-link:not(:last-child) {
    border-bottom: 1.5px solid var(--grey-100);
  }

  .settings-link:hover {
    background-color: var(--grey-50);
  }
</style>
