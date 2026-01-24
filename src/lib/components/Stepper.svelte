<!-- @component
  A stepper component for multi-step workflows.
  Displays numbered steps with labels, content area, and navigation buttons.
-->

<script lang="ts">
  import type { Snippet } from 'svelte';

  /**
   * Context passed to each step's content snippet to centralize data for each step.
   * We use 'unknown' to allow for steps to store any data. 
   * Users of Stepper should use type assertions to ensure type safety.
   */
  export interface StepContext {
    setData: (data: unknown) => void;
    getData: () => unknown | undefined;
    getAllData: () => Record<number, unknown>;
  }

  interface Step {
    name: string;
    content: Snippet<[StepContext]>;
    canProceed?: (data: unknown | undefined) => boolean;
  }

  interface Props {
    steps: Step[];
    onComplete?: (data: Record<number, unknown>) => void;
  }

  let { steps, onComplete }: Props = $props();

  let currentStep = $state(0);
  let stepData = $state<Record<number, unknown>>({});

  function setData(data: unknown) {
    stepData[currentStep] = data;
  }

  function getData(): unknown | undefined {
    return stepData[currentStep];
  }

  function getAllData(): Record<number, unknown> {
    // Shallow copy to prevent mutation of the original object
    return { ...stepData };
  }

  function goNext() {
    const currentStepConfig = steps[currentStep];
    const canProceed = currentStepConfig.canProceed?.(stepData[currentStep]) ?? true;
    
    if (!canProceed) {
      return;
    }

    if (currentStep < steps.length - 1) {
      currentStep++;
    } else if (onComplete) {
      onComplete(stepData);
    }
  }

  function goBack() {
    if (currentStep > 0) {
      currentStep--;
    }
  }
</script>

<div class="stepper">
  <!-- Step indicators -->
  <div class="step-indicators">
    {#each steps as step, index}
      <div class="step-item">
        <div class="step-circle-container">
          {#if index > 0}
            <div class="step-line step-line-left" class:completed={index <= currentStep}></div>
          {/if}
          <div
            class="step-circle paragraph-bold"
            class:completed={index < currentStep}
            class:current={index === currentStep}
            class:upcoming={index > currentStep}
          >
            {index + 1}
          </div>
          {#if index < steps.length - 1}
            <div class="step-line step-line-right" class:completed={index < currentStep}></div>
          {/if}
        </div>
        <span
          class="step-label paragraph"
          class:active={index <= currentStep}
        >
          {step.name}
        </span>
      </div>
    {/each}
  </div>

  <!-- Content area -->
  <div class="step-content">
    {@render steps[currentStep].content({ setData, getData, getAllData })}
  </div>

  <!-- Navigation -->
  <div class="step-navigation">
    <div class="nav-left">
      {#if currentStep > 0}
        <button class="nav-button paragraph" onclick={goBack}>
          <span class="nav-chevron">&lsaquo;</span> Back
        </button>
      {/if}
    </div>
    <div class="nav-right">
      <button class="nav-button paragraph" onclick={goNext}>
        {currentStep === steps.length - 1 ? 'Complete' : 'Next'} <span class="nav-chevron">&rsaquo;</span>
      </button>
    </div>
  </div>
</div>

<style>
  .stepper {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 900px;
    background-color: var(--background-white);
    border-radius: 16px;
    padding: 32px;
  }

  /* Step indicators */
  .step-indicators {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 48px;
  }

  .step-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    gap: 12px;
  }

  .step-circle-container {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: center;
    position: relative;
  }

  .step-circle {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    z-index: 1;
    transition: background-color 0.2s ease, color 0.2s ease;
  }

  .step-circle.completed {
    background-color: var(--grey-500);
    color: var(--grey-100);
  }

  .step-circle.current {
    box-shadow: inset 0 0 0 3px var(--grey-500);
    background-color: var(--grey-100);
    color: var(--grey-500);
  }

  .step-circle.upcoming {
    background-color: var(--grey-100);
    color: var(--grey-300);
  }

  .step-line {
    position: absolute;
    height: 4px;
    background-color: var(--grey-100);
    transition: background-color 0.2s ease;
  }

  .step-line-left {
    right: 50%;
    left: 0;
    margin-right: 24px;
  }

  .step-line-right {
    left: 50%;
    right: 0;
    margin-left: 24px;
  }

  .step-line.completed {
    background-color: var(--grey-500);
  }

  .step-label {
    text-align: center;
    color: var(--grey-200);
    transition: color 0.2s ease;
    max-width: 150px;
    font-size: 12px;
  }

  .step-label.active {
    color: var(--grey-500);
  }

  /* Content area */
  .step-content {
    flex: 1;
    min-height: 300px;
    padding: 16px 0;
  }

  /* Navigation */
  .step-navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 16px;
    border-top: 1px solid var(--grey-100);
  }

  .nav-left,
  .nav-right {
    min-width: 100px;
  }

  .nav-right {
    text-align: right;
  }

  .nav-button {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    color: var(--grey-500);
    cursor: pointer;
    padding: 8px 0;
    transition: color 0.15s ease;
    font-size: 14px;
  }

  .nav-button:hover {
    color: var(--grey-200);
  }

  .nav-chevron {
    font-size: 20px;
    line-height: 1;
  }
</style>
