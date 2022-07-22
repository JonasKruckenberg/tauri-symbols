<script lang="ts">
  import Icon from "../lib/Icon.svelte";
  import { getContext } from "svelte";
  const { getSymbols } = getContext("symbols");
  const symbols = getSymbols();
</script>

<section id="root">
  {#each symbols as symbol}
    <Icon icon={symbol} />
  {/each}
</section>

<style>
  section {
    --padding-top: 48px;
    --padding-bottom: 3em;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    gap: 1.5rem;
    height: calc(100vh - var(--padding-top) - var(--padding-bottom));
    overflow-y: scroll;
    padding: 0 3em;
    padding-top: var(--padding-top);
    padding-bottom: var(--padding-bottom);
  }
</style>
