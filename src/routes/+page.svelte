<script lang="ts">
  import SpendingBreakdown from "$lib/components/SpendingBreakdown.svelte";
  import TopCategories from "$lib/components/TopCategories.svelte";

  const spendingCategories = [
    { name: "Housing", amount: 555 },
    { name: "Healthcare", amount: 300 },
    { name: "Transportation", amount: 250 },
  ];

  const topCategories = [
    { name: "Restaurants", icon: "fluent:food-28-filled", spending: 555, budget: 600 },
    { name: "Healthcare", icon: "mdi:heart", spending: 84, budget: 400 },
    { name: "Transportation", icon: "bxs:car", spending: 400, budget: 600 },
    { name: "Subscriptions", icon: "fluent-mdl2:recurring-event", spending: 300, budget: 600 },
    { name: "Rent & Utilities", icon: "mdi:home", spending: 555, budget: 600 },
    { name: "Hobbies", icon: "mdi:palette", spending: 555, budget: 600 },
    { name: "Misc", icon: "mdi:wrench", spending: 555 },
  ];
</script>

<main class="container">
  <SpendingBreakdown categories={spendingCategories} />
  <TopCategories categories={topCategories} />
</main>

<style>
  .container {
    min-height: 100vh;
    display: flex;
    flex-grow: 1;
    align-items: center;
    justify-content: center;
    gap: 32px;
    padding: 32px;
  }
</style>
