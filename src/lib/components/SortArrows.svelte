<!-- @component
  Displays sort arrows for table column headers.
  Shows both arrows when null, up arrow when ascending, down arrow when descending.
-->

<script lang="ts" module>
  export type SortDirection = "asc" | "desc" | null;
</script>

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    column: string;
    activeColumn: string;
    direction: SortDirection;
  }

  let { column, activeColumn, direction }: Props = $props();

  const isActive = $derived(column === activeColumn);
  const showAsc = $derived(!isActive || direction === null || direction === "asc");
  const showDesc = $derived(!isActive || direction === null || direction === "desc");
</script>

<span class="arrows-container">
  {#if showAsc}
    <Icon 
      icon="bxs:up-arrow" 
      width={10} 
      height={10}
      class="arrow arrow-up"
    />
  {/if}
  {#if showDesc}
    <Icon 
      icon="bxs:up-arrow" 
      width={10} 
      height={10}
      class="arrow arrow-down"
    />
  {/if}
</span>

<style>
  .arrows-container {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  :global(.arrow) {
    color: var(--grey-500);
    transition: color 0.2s ease;
  }

  :global(.arrow-down) {
    transform: rotate(180deg);
  }
</style>
