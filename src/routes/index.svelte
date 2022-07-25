<script lang="ts">
  import Icon from "../lib/Icon.svelte";
  import { icons } from "../stores/icons";
</script>

<section>
  {#each $icons as icon}
    <Icon {icon} />
  {/each}
</section>

<style>
  section {
    --padding-top: 62px;
    --padding-bottom: 3em;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
    gap: 1.5rem;
    height: calc(100vh - var(--padding-top) - var(--padding-bottom));
    overflow-y: scroll;
    scroll-behavior: smooth;
    padding: 0 3em;
    padding-top: var(--padding-top);
    padding-bottom: var(--padding-bottom);
  }
</style>
