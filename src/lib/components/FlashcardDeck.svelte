<!-- @component
  A stacked "flashcard" deck with throw/recover animations. Renders up to 3 cards
  stacked on top of one another and animates the front card off-screen when advancing
  (throw) or back into place when reversing (recover). 
  ## Driving the deck
  Bind a reference and call its exported methods:

  ```svelte
  <FlashcardDeck
    bind:this={deck}
    bind:index
    bind:isAnimating
    {transactions}
    {card}
    {nextButton}
    {backButton}
  />
  ```
  Exports:
  - `deck.next(dir?)` — throws the current card out and advances `index` by 1.
  - `deck.back(dir?)` — steps `index` back by 1 and recovers the previous card in.

  `dir` is `"auto" | "left" | "right"` (default `"auto"`). `"auto"` alternates the throw
  direction by index parity. You can also pass an explicity animation direction. 
  
  ## Props
  - `transactions` — the full ordered list; the deck shows `index` plus the next two behind it.
  - `index` (bindable) — the current card's position; read it for a progress counter.
  - `isAnimating` (bindable) — true while a throw/recover is in flight.
  - `card` — `Snippet<[transaction]>` rendering a single card. Give each transaction a
    stable identity (id) so animations don't glitch when `index` changes.
  - `nextButton` / `backButton` — `Snippet<[]>` for the two controls.

  See src/routes/triage/+page.svelte for a working example.
-->

<script lang="ts">
  import type { FullTransaction } from "$lib/types/full_transaction";
  import type { Snippet } from "svelte";

  interface Props {
    transactions: FullTransaction[];
    index: number;
    isAnimating: boolean;
    card: Snippet<[FullTransaction]>;
    nextButton: Snippet<[]>;
    backButton: Snippet<[]>;
  }

  let {
    transactions,
    index = $bindable(0),
    isAnimating = $bindable(false),
    card,
    nextButton,
    backButton,
  }: Props = $props();

  interface Animation {
    card: Snippet<[FullTransaction]>;
    transaction: FullTransaction | null;
    direction: "left" | "right";
    mode: "out" | "in";
  }
  let animationParams = $state<Animation | null>(null);

  let totalCards = $derived(transactions.length);
  let currentTransaction = $derived(
    index < totalCards ? transactions[index] : null,
  );
  let isComplete = $derived(index >= totalCards);

  // Get up to 3 cards to display
  let visibleCards = $derived.by(() => {
    const cards: FullTransaction[] = [];
    for (let i = 0; i < 3 && index + i < totalCards; i++) {
      cards.push(transactions[index + i]);
    }
    return cards;
  });

  const getDirectionByIndex = (idx: number): "right" | "left" => {
    return idx % 2 == 0 ? "right" : "left";
  };
  export const next = (dir: "auto" | "left" | "right" = "auto") => {
    if (index >= transactions.length) {
      return;
    }
    isAnimating = true;

    if (dir === "auto") {
      dir = getDirectionByIndex(index);
    }
    animationParams = {
      card: card,
      transaction: currentTransaction,
      direction: dir,
      mode: "out",
    };

    index++;
  };
  export const back = (dir: "auto" | "left" | "right" = "auto") => {
    if (index == 0) {
      return;
    }
    isAnimating = true;
    index--;

    if (dir === "auto") {
      dir = getDirectionByIndex(index);
    }
    animationParams = {
      card: card,
      transaction: currentTransaction,
      direction: dir,
      mode: "in",
    };
  };

  const handleAnimationEnd = () => {
    isAnimating = false;
    animationParams = null;
  };
</script>

<div class="flashcard-deck">
  <div class="cards-container">
    {#if animationParams !== null && animationParams.transaction !== null}
      <div
        class="card-wrapper"
        class:throw-left={animationParams.direction === "left" &&
          animationParams.mode === "out"}
        class:throw-right={animationParams.direction === "right" &&
          animationParams.mode === "out"}
        class:recover-left={animationParams.direction === "left" &&
          animationParams.mode === "in"}
        class:recover-right={animationParams.direction === "right" &&
          animationParams.mode === "in"}
        onanimationend={handleAnimationEnd}
      >
        {@render animationParams.card(animationParams.transaction)}
      </div>
    {/if}
    {#if !isComplete}
      {#each visibleCards as visibleCard, stackPos (`${visibleCard.transaction.name}-${visibleCard.transaction.date}-${visibleCard.transaction.amount}-${visibleCard.transaction.id}`)}
        <div
          class="card-wrapper"
          class:card-back-2={stackPos === 2}
          class:card-back-1={stackPos === 1}
          class:card-front={stackPos === 0}
        >
          {@render card(visibleCard)}
        </div>
      {/each}
    {/if}
  </div>

  <div class="controls">
    {@render backButton()}

    <span class="counter">
      {isComplete ? totalCards : index + 1}/{totalCards}
    </span>

    {@render nextButton()}
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

  /* Recover animations */
  @keyframes recover-left {
    0% {
      transform: translate(-150px, -100px) rotate(-15deg);
      opacity: 0;
    }
    100% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
  }

  @keyframes recover-right {
    0% {
      transform: translate(150px, -100px) rotate(15deg);
      opacity: 0;
    }
    100% {
      transform: translateY(0) rotate(0deg);
      opacity: 1;
    }
  }

  .recover-left {
    animation: recover-left 0.35s ease-out forwards;
    z-index: 10;
  }

  .recover-right {
    animation: recover-right 0.35s ease-out forwards;
    z-index: 10;
  }
</style>
