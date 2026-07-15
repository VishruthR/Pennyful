<!-- @component
  A reusable text input component.
  Supports single-line (default) and multiline (textarea) modes.
  When multiline and maxlength are both set, a character counter is shown.
-->

<script lang="ts">
  interface Props {
    value?: string;
    placeholder?: string;
    multiline?: boolean;
    maxlength?: number;
    disabled?: boolean;
  }

  let {
    value = $bindable(''),
    placeholder = '',
    multiline = false,
    maxlength,
    disabled = false,
  }: Props = $props();
</script>

{#if multiline}
  <div class="wrapper" class:has-counter={maxlength != null}>
    <textarea
      class="input paragraph"
      bind:value
      {placeholder}
      {disabled}
      {maxlength}
    ></textarea>
    {#if maxlength != null}
      <span class="counter">{value.length}/{maxlength}</span>
    {/if}
  </div>
{:else}
  <input
    class="input paragraph"
    type="text"
    bind:value
    {placeholder}
    {disabled}
    {maxlength}
  />
{/if}

<style>
  .input {
    width: 100%;
    padding: 12px 16px;
    border-radius: 10px;
    border: 2px solid var(--grey-100);
    background-color: var(--pure-white);
    color: var(--grey-500);
    font-family: var(--font-family);
    font-size: 16px;
    font-weight: 400;
    transition: border-color 0.15s ease;
    outline: none;
    resize: none;
  }

  .input::placeholder {
    color: var(--grey-200);
  }

  .input:hover:not(:disabled) {
    border-color: var(--grey-200);
  }

  .input:focus:not(:disabled) {
    border-color: var(--grey-500);
  }

  .input:disabled {
    background-color: var(--grey-50);
    border-color: var(--grey-100);
    color: var(--grey-200);
    cursor: not-allowed;
  }

  .wrapper {
    position: relative;
    width: 100%;
  }

  .wrapper.has-counter .input {
    padding-bottom: 8px;
  }

  .counter {
    position: absolute;
    bottom: 12px;
    right: 16px;
    color: var(--grey-200);
    font-family: var(--font-family);
    font-size: 14px;
    font-weight: 400;
    pointer-events: none;
  }
</style>
