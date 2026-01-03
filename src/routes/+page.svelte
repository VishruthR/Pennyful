<script lang="ts">
  import SpendingBreakdown from "$lib/components/SpendingBreakdown.svelte";

  const spendingCategories = [
    { name: "Food", color: "#F2B834", icon: "fluent:food-28-filled", iconColor: "#F2B834", amount: 555 },
    { name: "Transportation", color: "#F4511E", icon: "bxs:car", iconColor: "#F4511E", amount: 300 },
    { name: "Subscriptions", color: "#FFA7A0", icon: "fluent-mdl2:recurring-event", iconColor: "#FFA7A0", amount: 250 },
    { name: "Healthcare", color: "#45CAAF", icon: "solar:health-bold", iconColor: "#45CAAF", amount: 150 },
    { name: "Entertainment", color: "#B585EC", icon: "fluent:movies-and-tv-20-filled", iconColor: "#B585EC", amount: 10 },
    { name: "Savings", color: "#AEAEAE", icon: "fluent:savings-32-filled", iconColor: "#AEAEAE", amount: 645 },
  ];
</script>

<main class="container">
  <SpendingBreakdown categories={spendingCategories} />
</main>

<style>
  :global(body) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    background-color: #f6f6f6;
    margin: 0;
  }

  .container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    box-sizing: border-box;
  }
</style>
