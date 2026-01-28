<!-- @component
  Displays a deck of transaction flashcards with discard/accept actions.
  Shows up to 3 cards stacked, with controls to process each card.
-->

<script lang="ts">
  import type { FullTransactionInfo } from "$lib/types";
  import Flashcard from "$lib/components/Flashcard.svelte";
  import Button from "$lib/components/Button.svelte";

  interface Props {
    transactions: FullTransactionInfo[];
    discardText?: string;
    acceptText?: string;
    onDiscard?: (transaction: FullTransactionInfo) => void;
    onAccept?: (transaction: FullTransactionInfo) => void;
    onComplete?: () => void;
  }

  let {
    transactions,
    discardText = "Delete",
    acceptText = "Submit",
    onDiscard,
    onAccept,
    onComplete,
  }: Props = $props();

  let currentIndex = $state(0);

  let totalCards = $derived(transactions.length);
  let currentTransaction = $derived(
    currentIndex < totalCards ? transactions[currentIndex] : null
  );
  let isComplete = $derived(currentIndex >= totalCards);

  // Get up to 3 cards to display
  let visibleCards = $derived(() => {
    const cards: FullTransactionInfo[] = [];
    for (let i = 0; i < 3 && currentIndex + i < totalCards; i++) {
      cards.push(transactions[currentIndex + i]);
    }
    return cards;
  });

  function handleDiscard() {
    if (currentTransaction) {
      const transaction = currentTransaction;
      currentIndex++;
      onDiscard?.(transaction);
      checkComplete();
    }
  }

  function handleAccept() {
    if (currentTransaction) {
      const transaction = currentTransaction;
      currentIndex++;
      onAccept?.(transaction);
      checkComplete();
    }
  }

  function checkComplete() {
    if (currentIndex >= totalCards) {
      onComplete?.();
    }
  }
</script>

<div class="flashcard-deck">
  <div class="cards-container">
    {#if !isComplete}
      {#each visibleCards() as card, index}
        <div
          class="card-wrapper"
          class:card-back-2={index === 2}
          class:card-back-1={index === 1}
          class:card-front={index === 0}
        >
          {#if index === 0}
            <Flashcard transaction={card} />
          {:else}
            <div class="card-placeholder"></div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>

  <div class="controls">
    <Button
      color="var(--loss-red)"
      onclick={handleDiscard}
      disabled={isComplete}
    >
      {discardText}
    </Button>

    <span class="counter">
      {isComplete ? totalCards : currentIndex + 1}/{totalCards}
    </span>

    <Button
      color="var(--profit-green)"
      onclick={handleAccept}
      disabled={isComplete}
    >
      {acceptText}
    </Button>
  </div>
</div>

<style>
  .flashcard-deck {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
  }

  .cards-container {
    position: relative;
    width: 400px;
    height: 280px;
  }

  .card-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    transition: transform 0.2s ease;
    box-shadow: 0 4px 20px 0 rgba(0, 0, 0, 0.3);
    border-radius: 12px;
  }

  .card-front {
    z-index: 3;
    transform: translateY(0);
  }

  .card-back-1 {
    z-index: 2;
    transform: translateY(-16px);
  }

  .card-back-2 {
    z-index: 1;
    transform: translateY(-32px);
  }

  .card-placeholder {
    width: 100%;
    height: 100%;
    background-color: var(--pure-white);
    border: 2px solid var(--grey-300);
    border-radius: 12px;
  }

  .controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    max-width: 400px;
    gap: 16px;
  }

  .counter {
    font-size: 16px;
    font-weight: 400;
    color: var(--grey-300);
  }
</style>
