<script lang="ts">
  import { goto } from "$app/navigation";
  import { plaidApi } from "$lib/api/plaid";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
    import { GITHUB_REPO_URL, openExternal, PLAID_TRIAL_URL, PLAID_URL } from "$lib/utils/externalLinks";
  import { load } from "@tauri-apps/plugin-store";

  const USERNAME_MAX_LENGTH = 80;

  let step = $state(0);

  // Step 1 state
  let username = $state("");
  let submittedUsername = $state<string | null>(null);

  const canSubmit = $derived(
    username.trim().length > 0 && username !== submittedUsername
  );
  const canProceed = $derived(
    username.trim().length > 0 && username === submittedUsername
  );

  const submitUsername = async () => {
    if (!canSubmit) {
      return;
    }

    const store = await load("store.json");
    await store.set("username", username);
    await store.save();

    submittedUsername = username;
  };

  const goToPlaidInfo = () => {
    if (!canProceed) {
      return;
    }
    step = 1;
  };

  // Step 2 state
  let clientId = $state("");
  let secret = $state("");

  const canProceedPlaidInfo = $derived(
    clientId.length > 0 && secret.length > 0
  );

  const onNext = async () => {
    if (!canProceedPlaidInfo) {
      return;
    }

    await plaidApi.savePlaidCredentials(clientId, secret);
    goto("/");
  };
</script>

<main class="onboarding">
  <div class="content">
    {#if step === 0}
      <h2 class="h2 title">Welcome to Pennyful!</h2>

      <div class="body">
        <p class="paragraph">
          Pennyful is an open-source, desktop budgeting app. We aim to give you
          insight and ownership of your finances.
        </p>
        <p class="paragraph">
          Most data is stored locally, on-device. No information is sent to
          Pennyful servers, in fact, Pennyful doesn't even run servers!
        </p>
        <p class="paragraph">
          The current beta of the app uses <a
            class="link"
            href={PLAID_URL}
            onclick={openExternal(PLAID_URL)}>Plaid</a
          > to ingest transactions. Plaid will store some of your banking data. If
          this is a concern, stay tuned for the next release which will allow for
          manual data entry (with no third-parties).
        </p>
        <p class="paragraph">
          Finally, since the code is open-source, you can audit it, suggest
          changes, or extend the project yourself <a
            class="link"
            href={GITHUB_REPO_URL}
            onclick={openExternal(GITHUB_REPO_URL)}>here.</a
          >
        </p>
      </div>

      <div class="username-row">
        <div class="username-field">
          <label class="paragraph" for="username">
            Let's start by creating a username:
          </label>
          <p class="hint">Be creative. No-one else will see this name.</p>
        </div>
        <div class="username-input">
          <TextInput
            id="username"
            bind:value={username}
            maxlength={USERNAME_MAX_LENGTH}
            showCounter
            placeholder="pennyful_user1234"
          />
        </div>
        <Button onclick={submitUsername} disabled={!canSubmit}>Submit</Button>
      </div>
    {:else}
      <h2 class="h2 title">Enter your plaid info</h2>

      <div class="body">
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
          You will need to enter your <strong>client_id</strong> and
          <strong>secret</strong>. This information is stored securely on your
          device's credentials manager (aka where your passwords are saved when
          you hit "Save Password"). These keys are only sent from your device to
          Plaid.
        </p>
      </div>

      <div class="fields">
        <div class="field">
          <label class="paragraph field-label" for="client_id">client_id:</label>
          <div class="field-input">
            <TextInput id="client_id" bind:value={clientId} placeholder="Input w/ Text" />
          </div>
        </div>
        <div class="field">
          <label class="paragraph field-label" for="secret">secret:</label>
          <div class="field-input">
            <TextInput id="secret" bind:value={secret} placeholder="Input w/ Text" />
          </div>
        </div>
      </div>

      <div class="body">
        <p class="paragraph step-note">
          Note: Since Pennyful stores credentials securely on your device's keychain, your device may prompt you for the device password whenever Pennyful tries to use these credentials. We recommend selecting "Always Allow" when your device prompts you for the password.
        </p>
      </div>
    {/if}
  </div>

  <div class="nav">
    <div class="nav-left">
      {#if step === 1}
        <button class="nav-button paragraph" onclick={() => (step = 0)}>
          <span class="nav-chevron">&lsaquo;</span> Back
        </button>
      {/if}
    </div>
    <div class="nav-right">
      {#if step === 0}
        <button
          class="nav-button paragraph"
          class:disabled={!canProceed}
          disabled={!canProceed}
          onclick={goToPlaidInfo}
        >
          Enter Plaid info <span class="nav-chevron">&rsaquo;</span>
        </button>
      {:else}
        <button
          class="nav-button paragraph"
          class:disabled={!canProceedPlaidInfo}
          disabled={!canProceedPlaidInfo}
          onclick={onNext}
        >
          Next <span class="nav-chevron">&rsaquo;</span>
        </button>
      {/if}
    </div>
  </div>
</main>

<style>
  .onboarding {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 64px 80px;
    background-color: var(--bg-white);
  }

  .content {
    width: 100%;
    max-width: 900px;
  }

  .title {
    margin: 0 0 40px;
  }

  .body {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 1160px;
  }

  .body .paragraph {
    margin: 0;
    color: var(--grey-300);
  }

  .link {
    color: inherit;
    text-decoration: underline;
    cursor: pointer;
  }

  .step-note {
    font-size: 12px;
    color: var(--grey-200);
  }

  /* Step 1: username */
  .username-row {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-top: 48px;
  }

  .username-field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .username-field label {
    margin-top: 6px;
    color: var(--grey-500);
  }

  .username-input {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .username-input :global(.input-wrapper) {
    width: 340px;
  }

  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--grey-200);
  }

  /* Step 2: plaid fields */
  .fields {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: 24px;
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
    width: 540px;
  }

  /* Navigation */
  .nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 1200px;
    margin: 48px auto 0;
    padding-top: 16px;
    border-top: 1px solid var(--grey-100);
  }

  .nav-left,
  .nav-right {
    min-width: 100px;
  }

  .nav-right {
    text-align: right;
  }

  .nav-button {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    color: var(--grey-500);
    cursor: pointer;
    padding: 8px 0;
    transition: color 0.15s ease;
    font-size: 14px;
  }

  .nav-button:hover:not(.disabled) {
    color: var(--grey-200);
  }

  .nav-button.disabled {
    color: var(--grey-200);
    cursor: not-allowed;
  }

  .nav-chevron {
    font-size: 20px;
    line-height: 1;
  }
</style>
