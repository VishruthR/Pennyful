<!-- @component
  A pie chart component that breaks down spending based on category. 
  It accepts an array of spending categories and displays them in an interactive pie chart.

  Example data:
  const spendingCategories = [
    { name: "Housing", amount: 555 },
    { name: "Transportation", amount: 300 },
    { name: "Healthcare", amount: 250 },
  ];

  const categoryDetails = {
    "Housing": { "name": "Housing", "color": "#B585EC", "icon": "gridicons:house" },
    "Transportation": { "name": "Transportation", "color": "#E67675", "icon": "mdi:car" },
    "Healthcare": { "name": "Healthcare", "color": "#7986CB", "icon": "solar:health-bold" },
  }
-->

<script lang="ts">
  import Icon from "@iconify/svelte";
  import type { Category } from "$lib/types.ts"
  import type { SpendingCategory } from "$lib/components/SpendingBreakdown.svelte"

  interface Segment extends SpendingCategory {
    percentage: number;
    path: string;
  }

  interface Props {
    categories: SpendingCategory[];
    categoryDetails: Map<String, Category>;
    size?: number;
    strokeWidth?: number;
  }

  const ICON_SIZE = 24;

  let { categories, categoryDetails, size = 200, strokeWidth = 20 }: Props = $props();
  let hoveredIndex = $state<number | null>(null);

  const center = $derived(size / 2);
  const radius = $derived((size - strokeWidth) / 2);
  const total = $derived(categories.reduce((sum, cat) => sum + cat.amount, 0));

  const polarToCartesian = (angle: number): { x: number; y: number } => {
    const rad = ((angle - 90) * Math.PI) / 180;
    return { x: center + radius * Math.cos(rad), y: center + radius * Math.sin(rad) };
  }

  const describeArc = (startAngle: number, endAngle: number): string => {
    const start = polarToCartesian(endAngle);
    const end = polarToCartesian(startAngle);
    const largeArc = endAngle - startAngle > 180 ? 1 : 0;
    return `M ${start.x} ${start.y} A ${radius} ${radius} 0 ${largeArc} 0 ${end.x} ${end.y}`;
  }

  const formatPercent = (pct: number): string => {
    return pct < 10 ? `${pct.toFixed(1)}%` : `${Math.round(pct)}%`;
  }

  const getCategoryDetails = (hoveredSegment: Segment | null): Category | undefined => {
    if (hoveredSegment === null) {
      return undefined;
    }

    console.log("hoveredSegment", hoveredSegment);
    console.log("specific category", categoryDetails.get(hoveredSegment.name));
    return categoryDetails.get(hoveredSegment.name);
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
  const hoveredDetails = $derived(
      getCategoryDetails(hoveredSegment)
  );

  const centerText = $derived.by(() => {
    if (hoveredSegment) {
      return {
        primary: hoveredSegment.name,
        secondary: `$${hoveredSegment.amount.toLocaleString()}`,
        tertiary: formatPercent(hoveredSegment.percentage),
      };
    }
    return {
      primary: "Total",
      secondary: `$${total.toLocaleString()}`,
      tertiary: null,
    };
  });

  const iconTransform = $derived(
    `translate(${center - ICON_SIZE / 2}, ${center - size * 0.26})`
  );

  $inspect(categoryDetails);
  $inspect("hoveredDetails", hoveredDetails);
</script>

<div class="pie-chart-container">
  <svg viewBox="0 0 {size} {size}" width={size} height={size} class="pie-chart" aria-label="Spending breakdown pie chart">
    <circle cx={center} cy={center} r={radius} fill="none" stroke="var(--grey-100)" stroke-width={strokeWidth} />

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
          <path d={segment.path} fill="none" stroke={getCategoryDetails(segment)?.color ?? "#111111"} stroke-width={strokeWidth} stroke-linecap="butt" />
        </g>
      {/each}

      {#if hoveredSegment && hoveredDetails}
        <g transform={iconTransform} class="segment-icon-wrapper" aria-hidden="true" style:color={hoveredDetails?.color ?? 'var(--grey-500)'}>
          <Icon icon={hoveredDetails.icon} width={ICON_SIZE} height={ICON_SIZE} />
        </g>
      {/if}

      <text x={center} y={center} class="center-text">
        <tspan x={center} dy="-0.5em" class="center-primary" style:fill={hoveredDetails?.color ?? 'var(--grey-500)'}>
          {centerText.primary}
        </tspan>
        <tspan x={center} dy="1.2em" class="center-secondary">{centerText.secondary}</tspan>
        {#if centerText.tertiary}
          <tspan x={center} dy="1.2em" class="center-tertiary" style:fill={hoveredDetails?.color ?? 'var(--grey-300)'}>
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
    font-family: var(--font-family);
  }

  .center-primary {
    font-size: 18px;
    font-weight: 500;
    fill: var(--grey-500);
  }

  .center-secondary {
    font-size: 24px;
    font-weight: 700;
    fill: var(--grey-500);
  }

  .center-tertiary {
    font-size: 18px;
    font-weight: 500;
    fill: var(--grey-300);
  }
</style>
