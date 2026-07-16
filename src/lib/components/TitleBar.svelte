<!-- @component
  Global top bar that displays the title of the current page.
-->

<script lang="ts">
  interface Props {
    title: string;
  }

  let { title }: Props = $props();
</script>

<header class="titlebar">
  <h1 class="titlebar-title paragraph-bold">{title}</h1>
</header>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    height: 73px;
    padding: 0 32px;
    border-bottom: 2px solid var(--grey-500);
    flex-shrink: 0;
  }

  .titlebar-title {
    margin: 0;
    color: var(--grey-500);
  }
</style>
