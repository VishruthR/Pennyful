<script lang="ts">
  import Icon from '@iconify/svelte';
  import { Combobox } from "melt/builders";
  import type { DropdownOption } from '$lib/types';
    import type { Snippet } from "svelte";

  interface Props {
    options: DropdownOption[];
    onSelect: (value: string | null) => void;
    placeholder?: string;
    getSelectedValue?: () => string;
  }

  let {
    options,
    onSelect,
    placeholder = 'Select...',
    getSelectedValue
  }: Props = $props();

  const optionsLabels = $derived(options.map(o => o.value));
  const optionsContent = $derived(Object.fromEntries(options.map(o => [o.value, o.content])));
  type Option = (typeof optionsLabels)[number];
 
  const combobox = new Combobox<Option>({
    value: getSelectedValue,
    onValueChange: (val) => {
      onSelect(val ?? null);
    },
  });

  let dropdownEl: HTMLDivElement;

  function handleWindowClick(e: MouseEvent) {
    if (combobox.open && dropdownEl && !dropdownEl.contains(e.target as Node)) {
      combobox.open = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="dropdown" bind:this={dropdownEl}>
  <button 
    class="dropdown-trigger paragraph {combobox.open ? 'open' : ''}"
    {...combobox.trigger}
  >
    <span class="dropdown-trigger-content">
      {#if combobox.value}
        {@render optionsContent[combobox.value]()}
      {:else}
        <span class="placeholder">{placeholder}</span>
      {/if}
    </span>
    <Icon icon="mdi:chevron-down"
      width={24}
      height={24}
      class="dropdown-chevron {combobox.touched ? 'rotated' : ''}"
    />
  </button>
  
  <ul
    class="dropdown-menu"
    {...combobox.content}
    popover={undefined}
  >
    {#each optionsLabels as option (option)}
      <li
        class="dropdown-item paragraph"
        {...combobox.getOption(option)}
      >
        {@render optionsContent[option]()}
      </li>
    {:else}
      <span class="paragraph">No results found</span>
    {/each}
  </ul>
</div>

<style>
  .dropdown {
    position: relative;
    width: 100%;
  }

  .dropdown-trigger {
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

  .dropdown-trigger:hover {
    border-color: var(--grey-200);
  }

  .dropdown-trigger:focus-visible {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .dropdown-trigger.open {
    border-color: var(--grey-300);
  }

  .dropdown-trigger-content {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .placeholder {
    color: var(--grey-200);
  }

  .dropdown-trigger :global(.dropdown-chevron) {
    flex-shrink: 0;
    color: var(--grey-200);
    transition: transform 0.2s ease;
  }

  .dropdown-trigger :global(.dropdown-chevron.rotated) {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    display: none;
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    margin: 0;
    padding: 8px 0;
    list-style: none;
    background-color: var(--pure-white);
    border: 2px solid var(--grey-300);
    border-radius: 10px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    max-height: 280px;
    overflow-y: auto;
    z-index: 100;
  }

  .dropdown-menu[data-open] {
    display: block;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    cursor: pointer;
    transition: background-color 0.1s ease;
  }

  .dropdown-item:hover,
  .dropdown-item.focused {
    background-color: var(--grey-50);
  }

  .dropdown-item.selected {
    background-color: var(--blue-50);
  }

</style>
