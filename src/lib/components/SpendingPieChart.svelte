<!-- @component
  A pie chart component that breaks down spending based on category. 
  It accepts an array of spending categories and displays them in an interactive pie chart.

  Example data:
  const spendingCategories = [
    { name: "Food", color: "#F2B834", icon: "fluent:food-28-filled", iconColor: "#F2B834", amount: 555 },
    { name: "Transportation", color: "#F4511E", icon: "bxs:car", iconColor: "#F4511E", amount: 300 },
    { name: "Subscriptions", color: "#FFA7A0", icon: "fluent-mdl2:recurring-event", iconColor: "#FFA7A0", amount: 250 },
    { name: "Healthcare", color: "#45CAAF", icon: "solar:health-bold", iconColor: "#45CAAF", amount: 150 },
    { name: "Entertainment", color: "#B585EC", icon: "fluent:movies-and-tv-20-filled", iconColor: "#B585EC", amount: 10 },
    { name: "Savings", color: "#AEAEAE", icon: "fluent:savings-32-filled", iconColor: "#AEAEAE", amount: 645 },
  ];
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface SpendingCategory {
    name: string;
    color: string;
    icon: string;
    iconColor: string;
    amount: number;
  }

  interface Segment extends SpendingCategory {
    percentage: number;
    path: string;
  }

  interface Props {
    categories: SpendingCategory[];
    size?: number;
    strokeWidth?: number;
    currency?: string;
  }

  const ICON_SIZE = 24;

  let { categories, size = 200, strokeWidth = 20, currency = "$" }: Props = $props();
  let hoveredIndex = $state<number | null>(null);

  const center = $derived(size / 2);
  const radius = $derived((size - strokeWidth) / 2);
  const total = $derived(categories.reduce((sum, cat) => sum + cat.amount, 0));

  function polarToCartesian(angle: number): { x: number; y: number } {
    const rad = ((angle - 90) * Math.PI) / 180;
    return { x: center + radius * Math.cos(rad), y: center + radius * Math.sin(rad) };
  }

  function describeArc(startAngle: number, endAngle: number): string {
    const start = polarToCartesian(endAngle);
    const end = polarToCartesian(startAngle);
    const largeArc = endAngle - startAngle > 180 ? 1 : 0;
    return `M ${start.x} ${start.y} A ${radius} ${radius} 0 ${largeArc} 0 ${end.x} ${end.y}`;
  }

  function formatPercent(pct: number): string {
    return pct < 10 ? `${pct.toFixed(1)}%` : `${Math.round(pct)}%`;
  }

  const segments: Segment[] = $derived.by(() => {
    let currentAngle = 0;
    return categories.map((cat) => {
      const percentage = total > 0 ? (cat.amount / total) * 100 : 0;
      const angleSpan = (percentage / 100) * 360;
      const path = describeArc(currentAngle, currentAngle + angleSpan);
      currentAngle += angleSpan;
      return { ...cat, percentage, path };
    });
  });

  const hoveredSegment = $derived(
    hoveredIndex !== null ? segments[hoveredIndex] ?? null : null
  );

  const centerText = $derived.by(() => {
    if (hoveredSegment) {
      return {
        primary: hoveredSegment.name,
        secondary: `${currency}${hoveredSegment.amount.toLocaleString()}`,
        tertiary: formatPercent(hoveredSegment.percentage),
      };
    }
    return {
      primary: "Total",
      secondary: `${currency}${total.toLocaleString()}`,
      tertiary: null,
    };
  });

  const iconTransform = $derived(
    `translate(${center - ICON_SIZE / 2}, ${center - size * 0.26})`
  );
</script>

<div class="pie-chart-container">
  <svg viewBox="0 0 {size} {size}" width={size} height={size} class="pie-chart" aria-label="Spending breakdown pie chart">
    <circle cx={center} cy={center} r={radius} fill="none" stroke="#e5e5e5" stroke-width={strokeWidth} />

    {#if categories.length === 0}
      <text x={center} y={center} class="center-text">
        <tspan x={center} dy="0.35em" class="center-secondary">No data</tspan>
      </text>
    {:else}
      {#each segments as segment, i}
        <g
          class="segment"
          class:hovered={hoveredIndex === i}
          style:transform-origin="{center}px {center}px"
          role="presentation"
          onmouseenter={() => (hoveredIndex = i)}
          onmouseleave={() => (hoveredIndex = null)}
        >
          <path d={segment.path} fill="none" stroke={segment.color} stroke-width={strokeWidth} stroke-linecap="butt" />
        </g>
      {/each}

      {#if hoveredSegment}
        <g transform={iconTransform} class="segment-icon-wrapper" aria-hidden="true" style:color={hoveredSegment.iconColor}>
          <Icon icon={hoveredSegment.icon} width={ICON_SIZE} height={ICON_SIZE} />
        </g>
      {/if}

      <text x={center} y={center} class="center-text">
        <tspan x={center} dy="-0.5em" class="center-primary" style:fill={hoveredSegment?.iconColor ?? '#333'}>
          {centerText.primary}
        </tspan>
        <tspan x={center} dy="1.2em" class="center-secondary">{centerText.secondary}</tspan>
        {#if centerText.tertiary}
          <tspan x={center} dy="1.2em" class="center-tertiary" style:fill={hoveredSegment?.iconColor ?? '#333'}>
            {centerText.tertiary}
          </tspan>
        {/if}
      </text>
    {/if}
  </svg>
</div>

<style>
  .pie-chart-container {
    display: inline-block;
  }

  .pie-chart {
    display: block;
    overflow: visible;
  }

  .segment {
    cursor: pointer;
    transition: transform 0.2s ease-in-out, filter 0.2s ease-in-out;
  }

  .segment.hovered {
    transform: scale(1.04);
    filter: drop-shadow(0 0 10px rgba(0, 0, 0, 0.4));
  }

  .segment-icon-wrapper {
    pointer-events: none;
  }

  .center-text {
    text-anchor: middle;
    font-family: inherit;
  }

  .center-primary {
    font-size: 1.1rem;
    font-weight: 500;
    fill: #333;
  }

  .center-secondary {
    font-size: 1.5rem;
    font-weight: 700;
    fill: #111;
  }

  .center-tertiary {
    font-size: 1.1rem;
    font-weight: 500;
    fill: #666;
  }
</style>
