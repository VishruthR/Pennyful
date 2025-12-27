<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SpendingPieChart from "$lib/components/SpendingPieChart.svelte";

  let name = $state("");
  let greetMsg = $state("");

  const spendingCategories = [
    { name: "Food", color: "#FFE062", icon: "fluent:food-28-filled", iconColor: "#CBA50B", amount: 555 },
    { name: "Savings", color: "#AEAEAE", icon: "fluent:savings-32-filled", iconColor: "#575757", amount: 645 },
    { name: "Transportation", color: "#FFA862", icon: "bxs:car", iconColor: "#CC742E", amount: 300 },
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
