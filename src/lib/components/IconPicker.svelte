<!-- @component
  A reusable icon picker form field, built on iconify-picker
-->

<script lang="ts">
  import Icon from '@iconify/svelte';
  import 'iconify-picker';

  interface Props {
    /** selected icon color, or null when nothing is chosen yet */
    iconName?: string | null;
    /** placeholder shown when no color is selected */
    placeholder?: string;
    /** input name, for native form submission */
    name?: string;
  }

  let { iconName = $bindable(null), placeholder = 'Choose an icon', name }: Props = $props();
  let dialogOpen = $state(false);
  let container: HTMLDivElement;
  let picker = $state<HTMLElement | null>(null);

  interface IconSelectedEvent {
    detail: {
      iconName: string;
    }
  };
  const handleIconSelected = (e: IconSelectedEvent) => {
    iconName = e.detail.iconName;
    dialogOpen = false;
  }

  // iconify-picker debounces search handler where it reads `e.target.value`
  // WebKit clears `event.target` as soon as event dispatch finishes, thus
  // the search term is removed before the search handler runs.
  // This workaround manually reads the search input and sets the attribute
  $effect(() => {
    const search = picker?.shadowRoot?.querySelector<HTMLInputElement>('.search');
    if (!search) return;
    let timer: ReturnType<typeof setTimeout>;
    search.oninput = () => {
      const value = search.value;
      clearTimeout(timer);
      timer = setTimeout(() => {
        (picker as any).state.currentPage = 0;
        picker!.setAttribute('search', value);
      }, 200);
    };
    return () => clearTimeout(timer);
  });

  // iconify-picker renders under a shadow DOM so we need to distinguish clicks
  // inside of it as focus and clicks outside of it and the trigger as unfocused
  $effect(() => {
    if (!dialogOpen) return;
    const handlePointerDown = (e: PointerEvent) => {
      if (!e.composedPath().includes(container)) {
        dialogOpen = false;
      }
    };
    document.addEventListener('pointerdown', handlePointerDown);
    return () => document.removeEventListener('pointerdown', handlePointerDown);
  });
</script>

<div class="container" bind:this={container}>
  <button
    class="trigger"
    onclick={() => dialogOpen = !dialogOpen}
  >
    {#if iconName === null}
      <span class="paragraph placeholder">{placeholder}</span>
      <Icon icon="mdi:chevron-down" width={24} height={24} />
    {:else}
      <span class="paragraph value">{iconName.split(":")[1]}</span>
      <Icon icon={iconName} width={24} />
    {/if}
  </button>
  <iconify-picker
    bind:this={picker}
    class="iconpicker"
    style:display={dialogOpen ? 'block' : 'none'}
    collection="mdi"
    height="300px"
    hide-collection
    onicon-selected={handleIconSelected}
    {name}
  >
    Pick and icon
  </iconify-picker>
</div>

<style>
  .container {
    width: 100%;
    position: relative;
  }

  .trigger {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 12px 12px 12px 16px;
    border: 2px solid var(--grey-100);
    border-radius: 10px;
    background-color: var(--pure-white);
    cursor: pointer;
    user-select: none;
    transition: border-color 0.15s ease;
  }

  .trigger:hover {
    border-color: var(--grey-200);
  }

  .trigger:focus-within {
    border-color: var(--grey-500);
  }

  .placeholder {
    color: var(--grey-200);
  }

  .placeholder:hover {
    color: var(--grey-500);
  }

  .value {
    font-size: 14px;
  }

  .trigger :global(svg) {
    flex-shrink: 0;
    color: var(--grey-200);
    transition: color 0.15s ease;
  }

  .trigger:hover :global(svg),
  .trigger:focus-within :global(svg) {
    color: var(--grey-500);
  }

  iconify-picker {
    width: 450px !important;
    max-width: 450px !important;
    height: 300px !important;
    --picker-primary: var(--grey-500);
    --picker-bg: var(--pure-white);
    --picker-text: var(--grey-500);
    --picker-border: var(--grey-500);
    --picker-hover: var(--grey-100);
    --picker-header-bg: var(--pure-white);
    --picker-input-bg: var(--pure-white);
    --picker-input-text: var(--grey-500);
    --picker-icon-color: var(--grey-500);
    --picker-icon-size: 20px;
  }

  .iconpicker {
    position: absolute;
    top: 60px;
  }
</style>
