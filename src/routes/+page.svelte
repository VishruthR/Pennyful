<script lang="ts">
  import FlashcardDeck from "$lib/components/FlashcardDeck.svelte";
  import type { FullTransactionInfo } from "$lib/types";

  const transactions: FullTransactionInfo[] = [
    {
      id: 1,
      name: "CSTCO 11/28 KIRKLAND SIGNATURE BLAH BLAH BLAH BLAH BLAH",
      amount: -190.10,
      date: new Date("2025-11-10"),
      account: { id: 1, name: "BOFA" },
      category: { id: 1, name: "Housing", color: "#A78BFA", icon: "mdi:home" },
    },
    {
      id: 2,
      name: "AMAZON PRIME MEMBERSHIP",
      amount: -14.99,
      date: new Date("2025-11-08"),
      account: { id: 2, name: "AMEX" },
      category: { id: 2, name: "Subscriptions", color: "#60A5FA", icon: "mdi:credit-card" },
    },
    {
      id: 3,
      name: "PAYROLL DEPOSIT",
      amount: 3500.00,
      date: new Date("2025-11-01"),
      account: { id: 1, name: "BOFA" },
      category: { id: 3, name: "Income", color: "#4ADE80", icon: "mdi:cash" },
    },
    {
      id: 4,
      name: "TRADER JOE'S #123",
      amount: -87.32,
      date: new Date("2025-11-05"),
      account: { id: 1, name: "BOFA" },
      category: { id: 4, name: "Groceries", color: "#FACC15", icon: "mdi:cart" },
    },
    {
      id: 5,
      name: "SHELL GAS STATION",
      amount: -52.40,
      date: new Date("2025-11-03"),
      account: { id: 2, name: "AMEX" },
      category: { id: 5, name: "Transportation", color: "#F97316", icon: "mdi:gas-station" },
    },
  ];

  function handleDiscard(transaction: FullTransactionInfo) {
    console.log('Discarded:', transaction.name);
  }

  function handleAccept(transaction: FullTransactionInfo) {
    console.log('Accepted:', transaction.name);
  }

  function handleComplete() {
    console.log('All cards processed!');
  }
</script>

<main class="container">
  <FlashcardDeck
    {transactions}
    discardText="Delete"
    acceptText="Submit"
    onDiscard={handleDiscard}
    onAccept={handleAccept}
    onComplete={handleComplete}
  />
</main>

<style>
  .container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px;
  }
</style>
