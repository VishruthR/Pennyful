<!-- @component
  A reusable button component.
  Supports full-width mode and form submission types.
-->

<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  interface Props extends HTMLButtonAttributes {
    fullWidth?: boolean;
    children: Snippet;
  }

  let {
    fullWidth = false,
    children,
    type = 'button',
    ...restProps
  }: Props = $props();
</script>

<button
  class="btn paragraph-bold"
  class:full-width={fullWidth}
  {type}
  {...restProps}
>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 16px 32px;
    border-radius: 10px;
    border: 2px solid var(--grey-300);
    background-color: var(--pure-white);
    color: var(--grey-500);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease, border-color 0.15s ease;
  }

  .btn:hover:not(:disabled) {
    background-color: var(--grey-300);
    border-color: var(--grey-300);
    color: var(--pure-white);
  }

  .btn:active:not(:disabled) {
    background-color: var(--grey-500);
    border-color: var(--grey-500);
    color: var(--pure-white);
  }

  .btn:focus-visible {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .btn:disabled {
    background-color: var(--grey-50);
    border-color: var(--grey-100);
    color: var(--grey-200);
    cursor: not-allowed;
  }

  .full-width {
    width: 100%;
  }
</style>
