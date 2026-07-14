<!-- @component
  A reusable single-line text input.
  Supports an optional inline character counter that enforces a `maxlength`.
-->

<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  interface Props extends HTMLInputAttributes {
    value?: string;
    maxlength?: number;
    showCounter?: boolean;
  }

  let {
    value = $bindable(''),
    maxlength,
    showCounter = false,
    ...restProps
  }: Props = $props();

  const hasCounter = $derived(showCounter && maxlength !== undefined);
</script>

<div class="input-wrapper">
  <input
    class="text-input paragraph"
    class:has-counter={hasCounter}
    bind:value
    {maxlength}
    {...restProps}
  />
  {#if hasCounter}
    <span class="counter">{value.length}/{maxlength}</span>
  {/if}
</div>

<style>
  .input-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
  }

  .text-input {
    width: 100%;
    padding: 12px 16px;
    border: 1.5px solid var(--grey-100);
    border-radius: 10px;
    background-color: var(--pure-white);
    color: var(--grey-500);
    font-family: var(--font-family);
    transition: border-color 0.15s ease;
  }

  .text-input.has-counter {
    padding-right: 56px;
  }

  .text-input::placeholder {
    color: var(--grey-200);
  }

  .text-input:focus {
    outline: none;
    border-color: var(--grey-300);
  }

  .counter {
    position: absolute;
    right: 16px;
    font-size: 12px;
    color: var(--grey-200);
    pointer-events: none;
  }
</style>
