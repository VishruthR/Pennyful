<script lang="ts">
  import Icon from "@iconify/svelte";

  interface SpendingCategory {
    name: string;
    color: string;
    icon: string;
    iconColor: string;
    amount: number;
  }

  interface Props {
    categories: SpendingCategory[];
    size?: number;
    strokeWidth?: number;
    currency?: string;
  }

  let { categories, size = 200, strokeWidth = 20, currency = "$" }: Props = $props();

  let hoveredIndex = $state<number | null>(null);

  const total = $derived(categories.reduce((sum, cat) => sum + cat.amount, 0));

  // Calculate the arc paths for each segment
  function polarToCartesian(cx: number, cy: number, r: number, angle: number) {
    const rad = ((angle - 90) * Math.PI) / 180;
    return {
      x: cx + r * Math.cos(rad),
      y: cy + r * Math.sin(rad),
    };
  }

  function describeArc(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
    const start = polarToCartesian(cx, cy, r, endAngle);
    const end = polarToCartesian(cx, cy, r, startAngle);
    const largeArcFlag = endAngle - startAngle <= 180 ? 0 : 1;

    return `M ${start.x} ${start.y} A ${r} ${r} 0 ${largeArcFlag} 0 ${end.x} ${end.y}`;
  }

  const segments = $derived.by(() => {
    const cx = size / 2;
    const cy = size / 2;
    const radius = (size - strokeWidth) / 2;
    let currentAngle = 0;

    return categories.map((cat, i) => {
      const percentage = total > 0 ? (cat.amount / total) * 100 : 0;
      const angleSpan = (percentage / 100) * 360;
      const startAngle = currentAngle;
      const endAngle = currentAngle + angleSpan;

      // Icon position at middle of arc
      const midAngle = startAngle + angleSpan / 2;
      const iconPos = polarToCartesian(cx, cy, radius, midAngle);

      currentAngle = endAngle;

      return {
        ...cat,
        percentage,
        startAngle,
        endAngle,
        path: describeArc(cx, cy, radius, startAngle, endAngle),
        iconX: iconPos.x,
        iconY: iconPos.y,
      };
    });
  });

  const centerText = $derived.by(() => {
    if (hoveredIndex !== null && segments[hoveredIndex]) {
      const seg = segments[hoveredIndex];
      return {
        primary: seg.name,
        secondary: `${currency}${seg.amount.toLocaleString()}`,
        tertiary: `${Math.round(seg.percentage)}%`,
      };
    }
    return {
      primary: "Total",
      secondary: `${currency}${total.toLocaleString()}`,
      tertiary: null,
    };
  });
</script>

<div class="pie-chart-container" style="--size: {size}px; --stroke-width: {strokeWidth}px;">
  <svg
    viewBox="0 0 {size} {size}"
    width={size}
    height={size}
    class="pie-chart"
  >
    <!-- Background ring -->
    <circle
      cx={size / 2}
      cy={size / 2}
      r={(size - strokeWidth) / 2}
      fill="none"
      stroke="#e5e5e5"
      stroke-width={strokeWidth}
    />

    <!-- Segment arcs -->
    {#each segments as segment, i}
      <g
        class="segment"
        class:hovered={hoveredIndex === i}
        role="button"
        tabindex="0"
        aria-label="{segment.name}: {currency}{segment.amount.toLocaleString()} ({Math.round(segment.percentage)}%)"
        onmouseenter={() => (hoveredIndex = i)}
        onmouseleave={() => (hoveredIndex = null)}
        onfocus={() => (hoveredIndex = i)}
        onblur={() => (hoveredIndex = null)}
      >
        <path
          d={segment.path}
          fill="none"
          stroke={segment.color}
          stroke-width={strokeWidth}
          stroke-linecap="butt"
          class="segment-path"
        />
        
        <!-- <foreignObject
          x={segment.iconX - 14}
          y={segment.iconY - 14}
          width="28"
          height="28"
          class="segment-icon-wrapper"
          aria-hidden="true"
        >
          <div class="icon-container" style="color: {segment.iconColor};">
            <Icon icon={segment.icon} width="24" height="24" />
          </div>
        </foreignObject> -->
      </g>
    {/each}

    <!-- Icon overlay when hovering -->
    {#if hoveredIndex !== null}
      <g
        transform="translate({size / 2 - 14}, {size / 2 - 52})"
        class="segment-icon-wrapper"
        aria-hidden="true"
        style="color: {segments[hoveredIndex].iconColor};"
      >
        <Icon icon={segments[hoveredIndex].icon} width="24" height="24" />
      </g>
    {/if}

    <!-- Center text -->
    <text x={size / 2} y={size / 2} class="center-text">
      <tspan x={size / 2} dy="-0.5em" class="center-primary" style="fill: {hoveredIndex !== null ? segments[hoveredIndex].iconColor : "#333"};">{centerText.primary}</tspan>
      <tspan x={size / 2} dy="1.4em" class="center-secondary">{centerText.secondary}</tspan>
      {#if centerText.tertiary}
        <tspan x={size / 2} dy="1.3em" class="center-tertiary" style="fill: {hoveredIndex !== null ? segments[hoveredIndex].iconColor : "#333"};">{centerText.tertiary}</tspan>
      {/if}
    </text>
  </svg>
</div>

<style>
  .pie-chart-container {
    display: inline-block;
    position: relative;
  }

  .pie-chart {
    display: block;
    overflow: visible;
  }

  .segment {
    cursor: pointer;
    outline: none;
    transition: transform 0.2s ease-in-out, filter 0.2s ease-in-out;
    transform-origin: center;
  }

  .segment.hovered {
    transform: scale(1.04);
    filter: drop-shadow(0 0 10px rgba(0, 0, 0, 0.4));
  }

  .segment-path {
    transition: all 0.2s ease-in-out;
  }

  .segment:focus {
    transform: scale(1.04);
    filter: drop-shadow(0 0 10px rgba(0, 0, 0, 0.4));
  }

  .segment-icon-wrapper {
    pointer-events: none;
    overflow: visible;
  }

  .icon-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
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

