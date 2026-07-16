<!-- @component
  A reusable color picker form field, built on svelte-awesome-color-picker.

  Displays an input-styled box ("Choose a color" + palette icon) that opens a color
  picker popup. Once a color is chosen, the box shows a small color circle and its hex
  code. Binds a hex string, e.g. `<ColorPicker bind:hex={category.color} />`.
-->

<script lang="ts">
  import Awesome from 'svelte-awesome-color-picker';
  import ColorPickerTrigger from './ColorPickerTrigger.svelte';

  interface Props {
    /** selected hex color, or null when nothing is chosen yet */
    hex?: string | null;
    /** placeholder shown when no color is selected */
    placeholder?: string;
    /** input name, for native form submission */
    name?: string;
  }

  let { hex = $bindable(null), placeholder = 'Choose a color', name }: Props = $props();
</script>

<div class="colorpicker">
  <Awesome
    bind:hex
    label={placeholder}
    {name}
    nullable
    isAlpha={false}
    position="responsive"
    components={{ input: ColorPickerTrigger }}
  />
</div>

<style>
  /* Styling to override colorpicker's default styles */
  .colorpicker {
    position: relative;
    width: 100%;

    --cp-bg-color: var(--pure-white);
    --cp-border-color: var(--grey-500);
    --cp-text-color: var(--grey-500);
    --focus-color: var(--grey-500);
  }

  .colorpicker :global(.color-picker) {
    display: block;
    width: 100%;
  }

  .colorpicker :global(.wrapper) {
    margin: 0;
    padding: 10px;
    border: 2px solid var(--grey-500);
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    z-index: 100;
  }

  .colorpicker :global(.nullability-checkbox) {
    display: none;
  }
</style>
