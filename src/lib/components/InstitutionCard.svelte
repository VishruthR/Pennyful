<!-- @component
  A slim, clickable card for a linked institution: icon + name + account count.
  The `addNew` variant renders a dashed "Link an institution" call to action.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";
  import { getInstitutionIcon } from "$lib/utils/institutionLogos";

  interface Props {
    name: string;
    institutionId?: string | null;
    accountCount?: number;
    selected?: boolean;
    loading?: boolean;
    addNew?: boolean;
    onClick: () => void;
  }
  let { name, institutionId = null, accountCount = 0, loading = false, selected = false, addNew = false, onClick }: Props = $props();

  const getDisplayText  = () => {
    if (loading) {
      return "Loading..."
    } else if (addNew) {
      return "Link an institution"
    } else {
      return name;
    }
  }
</script>

<button class="institution-card" class:selected class:add-new={addNew} onclick={onClick}>
  <Icon icon={addNew ? "mdi:bank-plus" : getInstitutionIcon(institutionId)} width={24} height={24} />
  <div class="institution-text">
    <span class="institution-name paragraph-bold">{getDisplayText()}</span>
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

  .institution-card:hover, .selected {
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

  .institution-card.add-new:hover {
    background-color: var(--blue-50);
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
