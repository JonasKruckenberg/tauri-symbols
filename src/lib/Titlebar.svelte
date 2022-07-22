<script lang="ts">
  import { scrollPosition } from "../stores/scroll-position";

  export let fontWeight = 400;
</script>

<header
  data-tauri-drag-region
  style={$scrollPosition && $scrollPosition > 0
    ? `box-shadow: 0px 6px 4px -6px rgba(0, 0, 0, 0.1)`
    : ""}
>
  <div />

  <div class="vstack">
    <span style="font-weight: bold;">All</span>
    <span>4145 Symbols</span>
  </div>

  <div />

  <select bind:value={fontWeight}>
    <option value={100}>Thin</option>
    <option value={200}>Ultra Light</option>
    <option value={300}>Light</option>
    <option value={400} default>Regular</option>
    <option value={500}>Medium</option>
    <option value={600}>Semi Bold </option>
    <option value={700}>Bold</option>
    <option value={800}>Heavy</option>
    <option value={900}>Black</option>
  </select>
</header>

<style>
  header {
    width: 100vw;
    position: sticky;
    height: 48px;
    display: grid;
    justify-content: space-between;
    align-items: center;
    transition: box-shadow 150ms;
    grid-template-columns: 80px max-content 1fr min-content 1fr;
  }

  header > * {
    user-select: none;
    pointer-events: none;
  }

  select {
    font-size: 1.2em;
  }

  select:focus {
    outline: none;
  }

  .vstack {
    font-size: .9rem;
    line-height: .9rem;
    display: flex;
    flex-direction: column;
  }
</style>
