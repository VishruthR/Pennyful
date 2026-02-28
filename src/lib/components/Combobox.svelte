<!-- @component
  Combobox which enables dropdown selection

  It accepts a list options with values and data + an optionRenderer
  Options are rendered through calling optionRenderer(option.data)
-->

<script lang="ts" generics="T">
  import { Select } from 'bits-ui';
  import Icon from '@iconify/svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    options: { value: string, data: T }[];
    optionRenderer: Snippet<[T]>;
    onSelect?: (value: string | null) => void;
    placeholder?: string;
    value?: string | null;
  }

  let { options, optionRenderer, onSelect, placeholder = 'Select...', value = $bindable<string | null>(null) }: Props = $props();

  let isOpen = $state(false);

  function handleValueChange(v: string | undefined) {
    value = v ?? null;
    onSelect?.(v ?? null);
  }

  const selectedOption = $derived(options.find(o => o.value === value));
</script>

<div class="combobox" class:open={isOpen}>
  <Select.Root
    type="single"
    value={value ?? undefined}
    onValueChange={handleValueChange}
    bind:open={isOpen}
  >
    <Select.Trigger class="trigger paragraph">
      <span class="trigger-content">
        {#if selectedOption}
          {@render optionRenderer(selectedOption.data)}
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
            {@render optionRenderer(option.data)} 
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
</style>
