<!-- @component
  Displays a category pill/badge with transparent text and icon.
  The text and icon use mix-blend-mode to show through to the underlying color.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    name: string;
    icon?: string;
    color: string;
  }

  let { name, icon, color }: Props = $props();
</script>

<span class="category-pill" style:background-color={color}>
  <span class="pill-content">
    {#if icon}
      <Icon {icon} width={14} height={14} />
    {/if}
    {name}
  </span>
</span>

<style>
  .category-pill {
    display: inline-flex;
    align-items: center;
    padding: 6px 12px;
    border-radius: 20px;
    white-space: nowrap;
  }

  .pill-content {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: bold;
    color: transparent;
  }
</style>
