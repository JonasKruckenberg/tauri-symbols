<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { icons } from "../stores/icons";
  import type { Icon } from "../stores/icons";
  import { scrollPosition } from "../stores/scroll-position";

  export let fontWeight = 400;

  function handleSearch(pattern: string) {
    if (pattern.trim().length == 0) {
      invoke<Icon[]>("all").then((data) => icons.set(data));
    } else {
      invoke<Icon[]>("search", { pattern }).then((data) => icons.set(data));
    }
  }
</script>

<header
  data-tauri-drag-region
  style={$scrollPosition && $scrollPosition > 0
    ? `box-shadow: 0px 6px 4px -6px rgba(0, 0, 0, 0.1)`
    : ""}
>
  <div />

  <div class="vstack">
    <span style="font-weight: bold;">All </span>
    <span>4145 Symbols</span>
  </div>

  <div />

  <input type="text" on:input={(event) => handleSearch(event.target.value)} />

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
    position: fixed;
    height: 48px;
    display: grid;
    justify-content: space-between;
    align-items: center;
    transition: box-shadow 150ms;
    grid-template-columns: 80px max-content 1fr min-content 1fr;
    z-index: 100;
    background-color: rgba(255, 255, 255, 0.9);
    -webkit-backdrop-filter: blur(10px);
    backdrop-filter: blur(10px);
  }

  header > * {
    user-select: none;
    pointer-events: none;
  }

  select,
  input {
    pointer-events: all;
  }

  select {
    font-size: 1.2em;
  }

  select:focus {
    outline: none;
  }

  .vstack {
    font-size: 0.9rem;
    line-height: 0.9rem;
    display: flex;
    flex-direction: column;
  }
</style>
