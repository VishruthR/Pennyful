<!-- @component
  Displays a category pill/badge with transparent text and icon.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    name: string;
    icon?: string;
    color: string;
    textColor: string;
  }

  let { name, icon, color, textColor = 'inherit' }: Props = $props();
</script>

<span class="category-pill" style:background-color={color} style:color={textColor}>
  <span class="pill-content">
    {#if icon}
      <Icon {icon} width={20} height={20} />
    {/if}
    {name}
  </span>
</span>

<style>
  .category-pill {
    display: inline-flex;
    align-items: center;
    padding: 6px 6px;
    border-radius: 10px;
    white-space: nowrap;
  }

  .pill-content {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: bold;
  }
</style>
