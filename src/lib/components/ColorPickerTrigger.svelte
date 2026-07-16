<!-- @component
  Custom trigger for the ColorPicker, passed to svelte-awesome-color-picker via
  `components={{ input: ColorPickerTrigger }}`.

  The parent library toggles the popup by listening for clicks that land inside
  `labelElement`, so the root must be a <label> bound to it. A visually-hidden native
  color input keeps the trigger keyboard-focusable and provides a form fallback.
-->

<script lang="ts">
  import Icon from '@iconify/svelte';

  interface Props {
    /** DOM element of the label wrapper — the library toggles the popup off this */
    labelElement?: HTMLLabelElement;
    /** selected hex color, or null when nothing is chosen yet */
    hex: string | null;
    /** placeholder label shown when no color is selected */
    label: string;
    /** input name, for native form submission */
    name?: string | undefined;
    /** directionality */
    dir: 'ltr' | 'rtl';
  }

  let { labelElement = $bindable(), hex, label, name = undefined, dir }: Props = $props();

  // Prevent the native browser color picker from opening; the library popup handles it.
  function preventDefault(e: MouseEvent) {
    e.preventDefault();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions, a11y_click_events_have_key_events -->
<label
  bind:this={labelElement}
  class="trigger"
  {dir}
  onclick={preventDefault}
  onmousedown={preventDefault}
>
  {#if hex}
    <span class="value">
      <span class="swatch" style:background={hex}></span>
      <span class="hex paragraph">{hex.toUpperCase()}</span>
    </span>
  {:else}
    <span class="placeholder paragraph">{label}</span>
  {/if}

  <input
    class="native"
    type="color"
    {name}
    value={hex}
    aria-haspopup="dialog"
    onclick={preventDefault}
    onmousedown={preventDefault}
  />

  <Icon icon="mdi:palette-outline" width={24} height={24} />
</label>

<style>
  .trigger {
    display: flex;
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

  .value {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .swatch {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 1.5px solid var(--grey-100);
  }

  .hex {
    color: var(--grey-300);
    text-transform: uppercase;
  }

  .placeholder {
    flex: 1;
    min-width: 0;
    color: var(--grey-200);
  }

  /* Visually hidden but focusable, for keyboard control and form fallback. */
  .native {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: 0;
    padding: 0;
    border: none;
    opacity: 0;
    pointer-events: none;
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
</style>
