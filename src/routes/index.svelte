<script lang="ts">
  import Icon from "../lib/Icon.svelte";
  import symbols from "../assets/data.json";
</script>

<section id="root">
  {#each symbols as symbol}
    <Icon icon={symbol} />
  {/each}
</section>

<style>
  section {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    grid-auto-rows: minmax(min-content, 9em);
    gap: 1.5rem;
    height: 100vh;
    overflow-y: scroll;
    padding: 0 3em;
    padding-top: 48px;
  }
</style>
