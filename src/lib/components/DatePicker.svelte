<!-- @component
  An accessible date picker built on bits-ui.
  Uses @internationalized/date for type-safe date handling.
-->

<script lang="ts">
  import { DatePicker } from 'bits-ui';
  import { CalendarDate, today, getLocalTimeZone } from '@internationalized/date';
  import Icon from '@iconify/svelte';
  import type { DateValue } from '@internationalized/date';

  interface Props {
    value?: Date | null;
    onchange?: (date: Date | null) => void;
    id?: string;
  }

  let { value = $bindable(null), onchange, id }: Props = $props();

  let isOpen = $state(false);

  // bits-ui requires CalendarDate; we derive it from the JS Date prop inline.
  const bitsValue = $derived(
    value ? new CalendarDate(value.getFullYear(), value.getMonth() + 1, value.getDate()) : undefined
  );

  function handleValueChange(v: DateValue | undefined) {
    value = v ? new Date(v.year, v.month - 1, v.day) : null;
    onchange?.(value);
  }

  // Use today's date as the placeholder so the calendar opens on the right month.
  const placeholder = today(getLocalTimeZone());
</script>

<div class="datepicker" class:open={isOpen}>
  <DatePicker.Root
    value={bitsValue}
    onValueChange={handleValueChange}
    bind:open={isOpen}
    {placeholder}
    weekStartsOn={0}
    weekdayFormat="short"
  >
    <DatePicker.Input {id} class="input">
      {#snippet children({ segments })}
        <div class="segments">
          {#each segments as { part, value: segVal }}
            {#if part === 'literal'}
              <DatePicker.Segment {part} class="segment literal">
                {segVal}
              </DatePicker.Segment>
            {:else}
              <DatePicker.Segment {part} class="segment">
                {segVal.toUpperCase()}
              </DatePicker.Segment>
            {/if}
          {/each}
        </div>

        <DatePicker.Trigger class="trigger" aria-label="Open calendar">
          <Icon icon="mdi:calendar-outline" width={24} height={24} />
        </DatePicker.Trigger>
      {/snippet}
    </DatePicker.Input>

    <DatePicker.Content class="popup" sideOffset={6}>
      <DatePicker.Calendar class="calendar">
        {#snippet children({ months, weekdays })}
          <DatePicker.Header class="cal-header">
            <DatePicker.PrevButton class="nav-btn" aria-label="Previous month">
              <Icon icon="mdi:chevron-left" width={24} height={24} />
            </DatePicker.PrevButton>
            <DatePicker.Heading class="cal-heading" />
            <DatePicker.NextButton class="nav-btn" aria-label="Next month">
              <Icon icon="mdi:chevron-right" width={24} height={24} />
            </DatePicker.NextButton>
          </DatePicker.Header>

          {#each months as month}
            <DatePicker.Grid class="cal-grid">
              <DatePicker.GridHead>
                <DatePicker.GridRow class="cal-row">
                  {#each weekdays as day}
                    <DatePicker.HeadCell class="day-label">
                      {day.slice(0, 2)}
                    </DatePicker.HeadCell>
                  {/each}
                </DatePicker.GridRow>
              </DatePicker.GridHead>

              <DatePicker.GridBody>
                {#each month.weeks as weekDates}
                  <DatePicker.GridRow class="cal-row">
                    {#each weekDates as date}
                      <DatePicker.Cell {date} month={month.value} class="cal-cell">
                        <DatePicker.Day class="day-btn">
                          {date.day}
                        </DatePicker.Day>
                      </DatePicker.Cell>
                    {/each}
                  </DatePicker.GridRow>
                {/each}
              </DatePicker.GridBody>
            </DatePicker.Grid>
          {/each}
        {/snippet}
      </DatePicker.Calendar>
    </DatePicker.Content>
  </DatePicker.Root>
</div>

<style>
  /* ── Wrapper (scopes all :global rules below to this component) ─────── */
  .datepicker {
    position: relative;
    width: 100%;
  }

  .datepicker :global(.input) {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px 12px 12px 16px;
    border: 2px solid var(--grey-100);
    border-radius: 10px;
    background-color: var(--pure-white);
    gap: 8px;
    cursor: text;
    transition: border-color 0.15s ease;
  }

  .datepicker :global(.input:hover) {
    border-color: var(--grey-200);
  }

  .datepicker :global(.input:focus-within),
  .datepicker.open :global(.input) {
    border-color: var(--grey-500);
  }

  .segments {
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 0;
    font-family: var(--font-family);
    font-size: 16px;
    font-weight: 400;
  }

  .datepicker :global(.segment) {
    display: inline-block;
    border-radius: 4px;
    padding: 0 1px;
    caret-color: transparent;
    outline: none;
    color: var(--grey-300);
  }

  .datepicker :global(.segment[data-placeholder]) {
    color: var(--grey-200);
  }

  .datepicker :global(.input:focus-within .segment[data-placeholder]),
  .datepicker :global(.input:focus-within .segment.literal) {
    color: var(--grey-300);
  }

  .datepicker :global(.segment:not(.literal):focus) {
    background-color: var(--blue-50);
    color: var(--grey-500);
  }

  .datepicker :global(.segment.literal) {
    color: var(--grey-200);
    user-select: none;
  }

  .datepicker :global(.trigger) {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    border: none;
    background: transparent;
    color: var(--grey-200);
    cursor: pointer;
    padding: 0;
    border-radius: 4px;
    transition: color 0.15s ease;
  }

  .datepicker :global(.trigger:hover),
  .datepicker.open :global(.trigger) {
    color: var(--grey-500);
  }

  .datepicker :global(.trigger:focus-visible) {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .datepicker :global(.popup) {
    background-color: var(--pure-white);
    border: 2px solid var(--grey-500);
    border-radius: 12px;
    padding: 10px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    z-index: 100;
  }

  .datepicker :global(.calendar) {
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-width: 268px;
  }

  .datepicker :global(.cal-header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .datepicker :global(.cal-heading) {
    font-family: var(--font-family);
    font-size: 18px;
    font-weight: 700;
    color: var(--grey-500);
    text-align: center;
    flex: 1;
  }

  .datepicker :global(.nav-btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--grey-500);
    cursor: pointer;
    border-radius: 6px;
    padding: 4px;
    transition: background-color 0.1s ease;
  }

  .datepicker :global(.nav-btn:hover) {
    background-color: var(--grey-50);
  }

  .datepicker :global(.nav-btn:focus-visible) {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .datepicker :global(.cal-grid) {
    width: 100%;
  }

  .datepicker :global(.cal-row) {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
  }

  .datepicker :global(.day-label) {
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-family);
    font-size: 14px;
    font-weight: 500;
    color: var(--grey-300);
    padding: 4px 0 8px;
  }

  .datepicker :global(.cal-cell) {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px 0;
  }

  .datepicker :global(.day-btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border: 2px solid transparent;
    border-radius: 8px;
    background: transparent;
    color: var(--grey-500);
    font-family: var(--font-family);
    font-size: 14px;
    font-weight: 400;
    cursor: pointer;
    transition: background-color 0.1s ease, border-color 0.1s ease;
  }

  .datepicker :global(.day-btn[data-outside-month]) {
    color: var(--grey-200);
  }

  .datepicker :global(.day-btn[data-today]:not([data-selected])) {
    border-color: var(--grey-300);
  }

  .datepicker :global(.day-btn:hover:not([data-selected]):not([data-disabled])) {
    background-color: var(--grey-50);
  }

  .datepicker :global(.day-btn:focus-visible) {
    outline: none;
    border-color: var(--grey-500);
  }

  .datepicker :global(.day-btn[data-selected]) {
    background-color: var(--grey-500);
    border-color: var(--grey-500);
    color: var(--pure-white);
    font-weight: 700;
  }

  .datepicker :global(.day-btn[data-disabled]) {
    color: var(--grey-100);
    cursor: not-allowed;
  }
</style>
