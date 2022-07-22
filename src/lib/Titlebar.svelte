<script lang="ts">
  import { scrollPosition } from "../stores/scroll-position";
  import { getContext } from "svelte";

  const { getSymbols } = getContext("symbols");
  const symbols = getSymbols();

  export let fontWeight = 400;
  export let fontColor;
  let numberOfSymbols: number = symbols.length;
</script>

<header
  data-tauri-drag-region
  style={$scrollPosition && $scrollPosition > 0
    ? `box-shadow: 0px 6px 4px -6px rgba(0, 0, 0, 0.1)`
    : ""}
>
  <!-- Leading edge -->
  <div class="vstack">
    <span style="font-weight: bold;">All </span>
    <span>{Intl.NumberFormat().format(numberOfSymbols)} Symbols</span>
  </div>

  <!-- Trailing edge -->
  <div class="trailing">
    <label for="colorpicker">Color Picker:</label>
    <input type="color" id="colorpicker" value="#ffffff" />
    <label for="font-weight">Symbol Weight</label>
    <select bind:value={fontWeight} id="font-weight">
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
  </div>
</header>

<style>
  :root {
    --titlebar-background: rgba(242, 242, 247, 0.9);
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --titlebar-background: rgba(28, 28, 30, 0.9);
    }
  }
  header {
    width: 100vw;
    position: fixed;
    height: 48px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: box-shadow 150ms;
    z-index: 100;
    background-color: var(--titlebar-background);
    -webkit-backdrop-filter: blur(20px);
    backdrop-filter: blur(10px);
    font-size: 0.9rem;
    line-height: 1.1rem;
  }

  header > * {
    user-select: none;
    pointer-events: none;
  }

  .vstack {
    display: flex;
    flex-direction: column;
    padding-left: 80px;
  }

  .trailing {
    padding-right: 2rem;
  }

  select {
    font-size: 1.2em;
    pointer-events: all;
  }

  select:focus {
    outline: none;
  }
</style>
