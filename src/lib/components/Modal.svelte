<!-- @component
  The app's standard modal, built on bits-ui `Dialog`. Provides an accessible
  dialog with a focus trap, Escape-to-close, scroll lock, and a click-dismissable
  backdrop. Renders a titled white card with a close button; caller supplies the body.

  Usage:
    <Modal bind:open title="Add Category">
      ...body...
    </Modal>

  The dialog is portalled to <body>, so its styles are declared `:global`.
-->

<script lang="ts">
  import { Dialog } from "bits-ui";
  import Icon from "@iconify/svelte";
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    title: string;
    children: Snippet;
  }

  let { open = $bindable(false), title, children }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-overlay" />
    <Dialog.Content class="modal-content" preventScroll>
      <div class="modal-header">
        <Dialog.Title class="modal-title h3">{title}</Dialog.Title>
        <Dialog.Close class="modal-close" aria-label="Close">
          <Icon icon="mdi:close" width={20} height={20} />
        </Dialog.Close>
      </div>
      {@render children()}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-overlay) {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.4);
    z-index: 200;
  }

  :global(.modal-content) {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 480px;
    max-width: calc(100vw - 48px);
    overflow: visible;
    padding: 32px;
    background-color: var(--pure-white);
    border-radius: 16px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.2);
    z-index: 201;
  }

  :global(.modal-content:focus) {
    outline: none;
  }

  :global(.modal-header) {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 24px;
  }

  :global(.modal-title) {
    margin: 0;
    color: var(--grey-500);
  }

  :global(.modal-close) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    background: none;
    border: none;
    border-radius: 6px;
    color: var(--grey-200);
    cursor: pointer;
    transition: color 0.15s ease, background-color 0.15s ease;
  }

  :global(.modal-close:hover) {
    color: var(--grey-500);
    background-color: var(--grey-50);
  }

  :global(.modal-close:focus-visible) {
    outline: 2px solid var(--grey-500);
    outline-offset: 1px;
  }
</style>
