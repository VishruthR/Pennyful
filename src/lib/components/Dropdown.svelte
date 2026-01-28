<!-- @component
  A reusable dropdown/select component.
  Supports custom option rendering via snippets, keyboard navigation, and click-outside-to-close.
-->

<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { DropdownOption } from '$lib/types';

  interface Props {
    options: DropdownOption[];
    onSelect: (value: string) => void;
    placeholder?: string;
    selectedValue?: string | null;
  }

  let {
    options,
    onSelect,
    placeholder = 'Select...',
    selectedValue = null,
  }: Props = $props();

  let isOpen = $state(false);
  let focusedIndex = $state(-1);
  let dropdownRef: HTMLDivElement | null = $state(null);
  let buttonRef: HTMLButtonElement | null = $state(null);
  let listRef: HTMLUListElement | null = $state(null);

  const selectedOption = $derived(
    options.find((opt) => opt.value === selectedValue)
  );

  function toggle() {
    isOpen = !isOpen;
    if (isOpen) {
      focusedIndex = selectedValue
        ? options.findIndex((opt) => opt.value === selectedValue)
        : 0;
    }
  }

  function close() {
    isOpen = false;
    focusedIndex = -1;
  }

  function selectOption(value: string) {
    onSelect(value);
    close();
    buttonRef?.focus();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!isOpen) {
      if (event.key === 'Enter' || event.key === ' ' || event.key === 'ArrowDown') {
        event.preventDefault();
        toggle();
      }
      return;
    }

    switch (event.key) {
      case 'Escape':
        event.preventDefault();
        close();
        buttonRef?.focus();
        break;
      case 'ArrowDown':
        event.preventDefault();
        focusedIndex = (focusedIndex + 1) % options.length;
        scrollToFocused();
        break;
      case 'ArrowUp':
        event.preventDefault();
        focusedIndex = focusedIndex <= 0 ? options.length - 1 : focusedIndex - 1;
        scrollToFocused();
        break;
      case 'Enter':
      case ' ':
        event.preventDefault();
        if (focusedIndex >= 0 && focusedIndex < options.length) {
          selectOption(options[focusedIndex].value);
        }
        break;
      case 'Tab':
        close();
        break;
      case 'Home':
        event.preventDefault();
        focusedIndex = 0;
        scrollToFocused();
        break;
      case 'End':
        event.preventDefault();
        focusedIndex = options.length - 1;
        scrollToFocused();
        break;
    }
  }

  function scrollToFocused() {
    if (listRef && focusedIndex >= 0) {
      const focusedItem = listRef.children[focusedIndex] as HTMLElement;
      focusedItem?.scrollIntoView({ block: 'nearest' });
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      close();
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside, true);
    } else {
      document.removeEventListener('click', handleClickOutside, true);
    }

    return () => {
      document.removeEventListener('click', handleClickOutside, true);
    };
  });
</script>

<div class="dropdown" bind:this={dropdownRef}>
  <button
    bind:this={buttonRef}
    type="button"
    class="dropdown-trigger paragraph"
    class:open={isOpen}
    class:has-selection={selectedOption}
    aria-haspopup="listbox"
    aria-expanded={isOpen}
    onclick={toggle}
    onkeydown={handleKeydown}
  >
    <span class="dropdown-trigger-content">
      {#if selectedOption}
        {@render selectedOption.content()}
      {:else}
        <span class="placeholder">{placeholder}</span>
      {/if}
    </span>
    <Icon
      icon="mdi:chevron-down"
      width={24}
      height={24}
      class="dropdown-chevron {isOpen ? 'rotated' : ''}"
    />
  </button>

  {#if isOpen}
    <ul
      bind:this={listRef}
      class="dropdown-menu"
      role="listbox"
      tabindex="-1"
      aria-activedescendant={focusedIndex >= 0 ? `option-${options[focusedIndex].value}` : undefined}
    >
      {#each options as option, index}
        <li
          id="option-{option.value}"
          class="dropdown-item paragraph"
          class:focused={focusedIndex === index}
          class:selected={selectedValue === option.value}
          role="option"
          aria-selected={selectedValue === option.value}
          onclick={() => selectOption(option.value)}
          onkeydown={(e) => e.key === 'Enter' && selectOption(option.value)}
          onmouseenter={() => (focusedIndex = index)}
        >
          {@render option.content()}
        </li>
      {/each}
    </ul>
  {/if}
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
