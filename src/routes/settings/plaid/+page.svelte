<script lang="ts">
  import { plaidApi } from "$lib/api/plaid";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { GITHUB_REPO_URL, openExternal, PLAID_TRIAL_URL } from "$lib/utils/externalLinks";

  type Status = { kind: "idle" | "saved" | "error"; message?: string };

  let clientId = $state("");
  let secret = $state("");

  let clientIdStatus = $state<Status>({ kind: "idle" });
  let secretStatus = $state<Status>({ kind: "idle" });

  const canSaveClientId = $derived(clientId.trim().length > 0);
  const canSaveSecret = $derived(secret.trim().length > 0);

  const saveClientId = async () => {
    if (!canSaveClientId) {
      return;
    }
    try {
      await plaidApi.savePlaidClientId(clientId);
      clientId = "";
      clientIdStatus = { kind: "saved" };
    } catch (e) {
      clientIdStatus = { kind: "error", message: String(e) };
    }
  };

  const saveSecret = async () => {
    if (!canSaveSecret) {
      return;
    }
    try {
      await plaidApi.savePlaidSecret(secret);
      secret = "";
      secretStatus = { kind: "saved" };
    } catch (e) {
      secretStatus = { kind: "error", message: String(e) };
    }
  };
</script>

<main class="page">
  <h2 class="h2">Plaid Credentials</h2>
  <div class="description">
    <p class="paragraph">
      For the current beta, the only way to add transactions is by syncing
      thru Plaid. Pennyful's Plaid integration can be offered for free
      through a bring-your-own-key model.
    </p>
    <p class="paragraph">
      Plaid offers a <a
        class="link"
        href={PLAID_TRIAL_URL}
        onclick={openExternal(PLAID_TRIAL_URL)}>trial plan</a
      > which allows users to connect 10 free financial institutions for free.
      This should be enough for most users, however, if you have issues with the
      trial plan, please open up an issue on the <a
        class="link"
        href={GITHUB_REPO_URL}
        onclick={openExternal(GITHUB_REPO_URL)}>GitHub repo</a
      >. You can sign up for the trial plan <a
        class="link"
        href={PLAID_TRIAL_URL}
        onclick={openExternal(PLAID_TRIAL_URL)}>here</a
      >.
    </p>
    <p class="paragraph">
      Update your Plaid <strong>client_id</strong> and <strong>secret</strong>. Each
      can be saved independently. New values replace the ones stored securely on your
      device's credentials manager.
    </p>
  </div>

  <div class="fields">
    <div class="field">
      <label class="paragraph field-label" for="client_id">client_id:</label>
      <div class="field-input">
        <TextInput id="client_id" bind:value={clientId} placeholder="Enter new client_id" />
      </div>
      <Button onclick={saveClientId} disabled={!canSaveClientId}>Save</Button>
      {#if clientIdStatus.kind === "saved"}
        <span class="status status-saved">Saved</span>
      {:else if clientIdStatus.kind === "error"}
        <span class="status status-error">{clientIdStatus.message}</span>
      {/if}
    </div>

    <div class="field">
      <label class="paragraph field-label" for="secret">secret:</label>
      <div class="field-input">
        <TextInput
          id="secret"
          type="password"
          bind:value={secret}
          placeholder="Enter new secret"
        />
      </div>
      <Button onclick={saveSecret} disabled={!canSaveSecret}>Save</Button>
      {#if secretStatus.kind === "saved"}
        <span class="status status-saved">Saved</span>
      {:else if secretStatus.kind === "error"}
        <span class="status status-error">{secretStatus.message}</span>
      {/if}
    </div>
  </div>

  <p class="paragraph step-note">
    Note: Since Pennyful stores credentials securely on your device's keychain, your
    device may prompt you for the device password whenever Pennyful tries to use these
    credentials. We recommend selecting "Always Allow" when your device prompts you for
    the password.
  </p>
</main>

<style>
  .page {
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .description {
    color: var(--grey-300);
    max-width: 720px;
  }

  .link {
    color: inherit;
    text-decoration: underline;
    cursor: pointer;
  }

  .fields {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: 24px 0;
  }

  .field {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .field-label {
    width: 80px;
    color: var(--grey-500);
  }

  .field-input {
    width: 480px;
  }

  .status {
    font-size: 14px;
  }

  .status-saved {
    color: var(--profit-green);
  }

  .status-error {
    color: var(--loss-red);
  }

  .step-note {
    font-size: 12px;
    color: var(--grey-200);
    max-width: 720px;
  }
</style>
