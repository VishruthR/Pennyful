<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SpendingPieChart from "$lib/components/SpendingPieChart.svelte";

  let name = $state("");
  let greetMsg = $state("");

  const spendingCategories = [
    { name: "Food", color: "#F2B834", icon: "fluent:food-28-filled", iconColor: "#F2B834", amount: 555 },
    { name: "Transportation", color: "#F4511E", icon: "bxs:car", iconColor: "#F4511E", amount: 300 },
    { name: "Subscriptions", color: "#FFA7A0", icon: "fluent-mdl2:recurring-event", iconColor: "#FFA7A0", amount: 250 },
    { name: "Healthcare", color: "#45CAAF", icon: "solar:health-bold", iconColor: "#45CAAF", amount: 150 },
    { name: "Entertainment", color: "#B585EC", icon: "fluent:movies-and-tv-20-filled", iconColor: "#B585EC", amount: 10 },
    { name: "Savings", color: "#AEAEAE", icon: "fluent:savings-32-filled", iconColor: "#AEAEAE", amount: 645 },
  ];

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="chart-section">
    <h2>Monthly Spending</h2>
    <SpendingPieChart categories={spendingCategories} />
  </div>

</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #f6f6f6;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}

.chart-section {
  margin-bottom: 2rem;
}

.chart-section h2 {
  margin-bottom: 1rem;
  font-size: 1.25rem;
  color: #555;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #f6f6f6;
  }
}

</style>
