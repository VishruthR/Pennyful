<!-- @component
  Displays a deck of transaction flashcards with discard/accept actions.
  Shows up to 3 cards stacked, with controls to process each card.
-->

<script lang="ts">
  import type { TransactionImport } from "$lib/types";
  import Flashcard from "$lib/components/Flashcard.svelte";
  import Button from "$lib/components/Button.svelte";

  interface Props {
    transactions: TransactionImport[];
    discardText?: string;
    acceptText?: string;
    onDiscard: (transaction: TransactionImport) => void;
    onAccept: (transaction: TransactionImport) => void;
    onComplete: () => void;
  }

  let {
    transactions,
    discardText = "Delete",
    acceptText = "Accept",
    onDiscard,
    onAccept,
    onComplete,
  }: Props = $props();

  let currentIndex = $state(0);
  let throwDirection = $state<"left" | "right" | null>(null);
  let isAnimating = $state(false);

  let totalCards = $derived(transactions.length);
  let currentTransaction = $derived(
    currentIndex < totalCards ? transactions[currentIndex] : null
  );
  let isComplete = $derived(currentIndex >= totalCards);

  // Get up to 3 cards to display
  let visibleCards = $derived.by(() => {
    const cards: TransactionImport[] = [];
    for (let i = 0; i < 3 && currentIndex + i < totalCards; i++) {
      cards.push(transactions[currentIndex + i]);
    }
    return cards;
  });

  function handleAnimationEnd() {
    if (throwDirection && currentTransaction) {
      const transaction = currentTransaction;
      const direction = throwDirection;

      currentIndex++;
      throwDirection = null;
      isAnimating = false;

      if (direction === "left") {
        onDiscard(transaction);
      } else {
        onAccept(transaction);
      }
      checkComplete();
    }
  }

  function handleDiscard() {
    if (currentTransaction && !isAnimating) {
      isAnimating = true;
      throwDirection = "left";
    }
  }

  function handleAccept() {
    if (currentTransaction && !isAnimating) {
      isAnimating = true;
      throwDirection = "right";
    }
  }

  function checkComplete() {
    if (currentIndex >= totalCards) {
      onComplete();
    }
  }
</script>

<div class="flashcard-deck">
  <div class="cards-container">
    {#if !isComplete}
      {#each visibleCards as card, index (`${card.name}-${card.date}-${card.amount}-${index}`)}
        <div
          class="card-wrapper"
          class:card-back-2={index === 2}
          class:card-back-1={index === 1}
          class:card-front={index === 0}
          class:throw-left={index === 0 && throwDirection === "left"}
          class:throw-right={index === 0 && throwDirection === "right"}
          onanimationend={index === 0 ? handleAnimationEnd : undefined}
        >
          <Flashcard transaction={card} />
        </div>
      {/each}
    {/if}
  </div>

  <div class="controls">
    <Button
      color="var(--loss-red-50)"
      onclick={handleDiscard}
      disabled={isComplete || isAnimating}
    >
      {discardText}
    </Button>

    <span class="counter">
      {isComplete ? totalCards : currentIndex + 1}/{totalCards}
    </span>

    <Button
      color="var(--profit-green-50)"
      onclick={handleAccept}
      disabled={isComplete || isAnimating}
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
    flex: 1;
    gap: 24px;
  }

  .cards-container {
    position: relative;
    width: 100%;
    height: 280px;
  }

  .card-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    transition: transform 0.2s ease-out;
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

  /* Throw animations */
  @keyframes throw-left {
    0% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
    100% {
      transform: translate(-150px, -100px) rotate(-15deg);
      opacity: 0;
    }
  }

  @keyframes throw-right {
    0% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
    100% {
      transform: translate(150px, -100px) rotate(15deg);
      opacity: 0;
    }
  }

  .throw-left {
    animation: throw-left 0.35s ease-out forwards;
    z-index: 10;
  }

  .throw-right {
    animation: throw-right 0.35s ease-out forwards;
    z-index: 10;
  }
</style>
