<!-- @component
  Combobox that allows users to select from a list of options

  It allows caller to pass in a list of items as well as Snippets to render items
  However, an optional item prop can also be passed in which will override the options.content
  field. This item prop is convenient when a user wants to render a similar element 
  but with modifications based on the content of option.item
-->

<script lang="ts">
  import { Select } from 'bits-ui';
  import Icon from '@iconify/svelte';
  import type { Snippet } from 'svelte';
  import type { DropdownOption } from '$lib/types';

  interface Props {
    options: DropdownOption[];
    onSelect?: (value: string | null) => void;
    placeholder?: string;
    value?: string | null;
    variant?: 'default' | 'slim';
    item?: Snippet<[DropdownOption]>;
  }

  let {
    options,
    onSelect,
    placeholder = 'Select...',
    value = $bindable<string | null>(null),
    variant = 'default',
    item
  }: Props = $props();

  let isOpen = $state(false);

  function handleValueChange(v: string | undefined) {
    value = v ?? null;
    onSelect?.(v ?? null);
  }

  const selectedOption = $derived(options.find(o => o.value === value));
  const optionsContent = $derived(Object.fromEntries(options.map(o => [o.value, o.content])));
</script>

<div class="combobox" class:open={isOpen} class:slim={variant === 'slim'}>
  <Select.Root
    type="single"
    value={value ?? undefined}
    onValueChange={handleValueChange}
    bind:open={isOpen}
  >
    <Select.Trigger class="trigger paragraph">
      <span class="trigger-content">
        {#if selectedOption}
          {#if item}
            {@render item(selectedOption)}
          {:else}
            {@render selectedOption.content?.()}
          {/if}
        {:else}
          <span class="placeholder">{placeholder}</span>
        {/if}
      </span>
      <Icon icon="mdi:chevron-down" width={24} height={24} class="chevron" />
    </Select.Trigger>

    <Select.Content class="menu" sideOffset={4}>
      <Select.Viewport>
        {#each options as option (option.value)}
          <Select.Item value={option.value} label={option.value} class="item paragraph">
            {#if item}
              {@render item(option)}
            {:else}
              {@render optionsContent[option.value]?.()}
            {/if}
          </Select.Item>
        {:else}
          <span class="paragraph">No results found</span>
        {/each}
      </Select.Viewport>
    </Select.Content>
  </Select.Root>
</div>

<style>
  /* ── Wrapper (scopes all :global rules to this component) ──────────── */
  .combobox {
    position: relative;
    width: 100%;
  }

  .combobox :global(.trigger) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 12px 16px;
    border-radius: 10px;
    border: 2px solid var(--grey-100);
    background-color: var(--pure-white);
    color: var(--grey-500);
    cursor: pointer;
    transition: border-color 0.15s ease;
    text-align: left;
  }

  .combobox :global(.trigger:hover) {
    border-color: var(--grey-200);
  }

  .combobox :global(.trigger:focus-visible) {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .combobox.open :global(.trigger) {
    border-color: var(--grey-300);
  }

  .trigger-content {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .placeholder {
    color: var(--grey-200);
  }

  .combobox :global(.chevron) {
    flex-shrink: 0;
    color: var(--grey-200);
    transition: transform 0.2s ease;
  }

  .combobox.open :global(.chevron) {
    transform: rotate(180deg);
    color: var(--grey-300);
  }

  .combobox :global(.menu) {
    width: var(--bits-select-anchor-width);
    background-color: var(--pure-white);
    border: 2px solid var(--grey-300);
    border-radius: 10px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    max-height: 280px;
    overflow-y: auto;
    z-index: 100;
    padding: 8px 0;
  }

  .combobox :global(.item) {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    cursor: pointer;
    transition: background-color 0.1s ease;
  }

  .combobox :global(.item[data-highlighted]) {
    background-color: var(--grey-50);
  }

  .combobox :global(.item[data-selected]) {
    background-color: var(--blue-50);
  }

  /* ── Slim variant: Combobox with minimal padding ─────── */
  .combobox.slim :global(.trigger) {
    padding: 4px 4px;
  }
</style>
