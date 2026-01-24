<!-- @component
  A clickable card displaying bank account information.
  Shows account icon, name, provider, type, and balance.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    icon: string;
    name: string;
    accountType: string;
    provider: string;
    balance: number;
    onClick: () => void;
  }

  let { icon, name, accountType, provider, balance, onClick }: Props = $props();

  const isNegative = $derived(balance < 0);

  const formattedBalance = $derived(
    balance.toLocaleString('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })
  );
</script>

<button class="bank-account-card" onclick={onClick}>
  <div class="card-header">
    <Icon {icon} width={24} height={24} />
    <span class="account-name h3">{name}</span>
  </div>
  <div class="account-details paragraph">
    {provider} &bull; {accountType}
  </div>
  <div class="balance-row paragraph">
    <span class="balance-label">Balance:</span>
    <span class="balance-amount" class:negative={isNegative}>{formattedBalance}</span>
  </div>
</button>

<style>
  .bank-account-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    width: 100%;
    padding: 16px 20px;
    border-radius: 10px;
    border: 2px solid var(--grey-300);
    background-color: var(--pure-white);
    cursor: pointer;
    transition: background-color 0.15s ease;
    text-align: left;
    font-family: inherit;
  }

  .bank-account-card:hover {
    background-color: var(--blue-50);
  }

  .bank-account-card:focus-visible {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--grey-500);
  }

  .account-name {
    color: var(--grey-500);
  }

  .account-details {
    color: var(--grey-200);
  }

  .balance-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;
  }

  .balance-label {
    color: var(--grey-500);
  }

  .balance-amount {
    color: var(--profit-green);
    font-weight: 500;
  }

  .balance-amount.negative {
    color: var(--loss-red);
  }
</style>
