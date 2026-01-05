<script lang="ts">
  import SpendingBreakdown from "$lib/components/SpendingBreakdown.svelte";
  import TopCategories from "$lib/components/TopCategories.svelte";

  const spendingCategories = [
    { name: "Food", color: "#F2B834", icon: "fluent:food-28-filled", iconColor: "#F2B834", amount: 555 },
    { name: "Transportation", color: "#F4511E", icon: "bxs:car", iconColor: "#F4511E", amount: 300 },
    { name: "Subscriptions", color: "#FFA7A0", icon: "fluent-mdl2:recurring-event", iconColor: "#FFA7A0", amount: 250 },
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
