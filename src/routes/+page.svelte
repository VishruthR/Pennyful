<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import TransactionsTable from "$lib/components/TransactionsTable.svelte";
  import type { FullTransactionInfo } from "$lib/types";

  const transactions: FullTransactionInfo[] = [
    {
      id: 1,
      name: "COSTCO WHSE #0008 11/1 PURCHASE...",
      amount: -10.99,
      date: new Date("2025-11-01"),
      account: { id: 1, name: "BOFA" },
      category: { id: 6, name: "Food", color: "#FEEE8C", icon: "fluent:food-16-filled" }
    },
    {
      id: 2,
      name: "SimpleBills Prod DES:WEB PMTS ID:XX...",
      amount: -100.99,
      date: new Date("2025-11-01"),
      account: { id: 2, name: "Fidelity" },
      category: { id: 5, name: "Transportation", color: "#E67675", icon: "mdi:car" }
    },
    {
      id: 3,
      name: "Zelle payment from JOHN DOE for 'food'; Conf# 11bu1u1",
      amount: -100.99,
      date: new Date("2025-11-01"),
      account: { id: 3, name: "Wells Fargo" },
      category: { id: 2, name: "Housing", color: "#B585EC", icon: "gridicons:house" }
    },
    {
      id: 4,
      name: "WELLS FARGO CARD DES:CRD ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB",
      amount: -100.99,
      date: new Date("2025-10-31"),
      account: { id: 1, name: "BOFA" },
      category: { id: 13, name: "Fun", color: "#45CAAF", icon: "bxs:party" }
    },
    {
      id: 5,
      name: "COSTCO WHSE#1111 11/14 PURCHASE CITY ST",
      amount: 100.99,
      date: new Date("2025-10-30"),
      account: { id: 4, name: "Amex" },
      category: { id: 7, name: "Savings", color: "#AEAEAE", icon: "fluent:savings-24-filled" }
    },
    {
      id: 6,
      name: "BILTPYMTS DES:RENT PMT ID:XXXXXXXXXX INDN:JOHN DOE CO ID:XXXXX WEB",
      amount: -100.99,
      date: new Date("2025-10-15"),
      account: { id: 5, name: "Some other bank account...." },
      category: { id: 9, name: "Healthcare", color: "#7986CB", icon: "solar:health-bold" }
    },
    {
      id: 7,
      name: "AMEX Airline Fee Reimbursement",
      amount: -100.99,
      date: new Date("2025-10-13"),
      account: { id: 6, name: "Venmo" },
      category: { id: 11, name: "Hobbies", color: "#0493D9", icon: "mdi:youtube-sports" }
    },
    {
      id: 8,
      name: "FOOD MARKET #111 CITY STATE",
      amount: -100.99,
      date: new Date("2025-09-28"),
      account: { id: 2, name: "Fidelity" },
      category: { id: 12, name: "Miscellaneous", color: "#3F51B5", icon: "mdi:tools" }
    },
    {
      id: 9,
      name: "SQ *FOOD GRILL CITY STATE",
      amount: -100.99,
      date: new Date("2025-08-18"),
      account: { id: 1, name: "BOFA" },
      category: { id: 1, name: "Income", color: "#CEE3BE", icon: "vaadin:money" }
    }
  ];
</script>

<main class="container">
  <div class="button-demo">
    <h2 class="h3">Button Variants</h2>
    <div class="button-row">
      <Button onclick={() => console.log('Normal button clicked')}>
        Add transaction
      </Button>
      <Button disabled>
        Add transaction
      </Button>
    </div>
  </div>
  <TransactionsTable {transactions} />
</main>

<style>
  .container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    align-items: center;
    gap: 32px;
    padding: 32px;
  }

  .button-demo {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .button-row {
    display: flex;
    gap: 16px;
    align-items: center;
  }
</style>
