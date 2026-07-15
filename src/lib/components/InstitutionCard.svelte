<!-- @component
  A slim, clickable card for a linked institution: icon + name + account count.
  The `addNew` variant renders a dashed "Link an institution" call to action.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    name: string;
    accountCount?: number;
    selected?: boolean;
    addNew?: boolean;
    onClick: () => void;
  }
  let { name, accountCount = 0, selected = false, addNew = false, onClick }: Props = $props();
</script>

<button class="institution-card" class:selected class:add-new={addNew} onclick={onClick}>
  <Icon icon={addNew ? "mdi:bank-plus" : "mdi:bank"} width={24} height={24} />
  <div class="institution-text">
    <span class="institution-name paragraph-bold">{addNew ? "Link an institution" : name}</span>
    {#if !addNew}
      <span class="institution-subline">
        {accountCount} account{accountCount === 1 ? "" : "s"} added
      </span>
    {/if}
  </div>
</button>

<style>
  .institution-card {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 12px 16px;
    border-radius: 10px;
    border: 2px solid var(--grey-200);
    background-color: var(--pure-white);
    cursor: pointer;
    transition: background-color 0.15s ease;
    text-align: left;
    font-family: inherit;
    color: var(--grey-500);
  }

  .institution-card:hover {
    background-color: var(--blue-50);
  }

  .institution-card.selected {
    border: 2px solid var(--grey-300);
  }

  .institution-card.add-new {
    border: 2px dashed var(--grey-200);
    background-color: transparent;
    color: var(--grey-300);
  }

  .institution-card:focus-visible {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .institution-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .institution-name {
    color: inherit;
  }

  .institution-subline {
    font-size: 12px;
    color: var(--grey-200);
  }
</style>
