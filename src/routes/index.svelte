<script lang="ts">
  import VirtualList from "../lib/VirtualList.svelte";
  import Icon from "../lib/Icon.svelte";
  import symbols from "../assets/data.json";
  import { scrollPosition } from '../stores/scroll-position'

  let itemsPerRow = 7;
  // New array that contains numbers 1..n
  // Dividing the data array length by 6 as there are 6 items per row
  let items = [...Array(Math.ceil(symbols.length / itemsPerRow)).keys()];

  let end: number = Math.floor(750 / itemsPerRow);
</script>

<section style={`--items-per-row: ${itemsPerRow};`}>
  <VirtualList height="90vh" {items} let:item bind:start={$scrollPosition} bind:end>
    <div class="row">
      <!-- loop for items in each row, that references 0..5, then 6..11 etc -->
      {#each Array(itemsPerRow) as _, i}
        {#if symbols[item * itemsPerRow + i]}
          <Icon icon={symbols[item * itemsPerRow + i]} />
        {/if}
      {/each}
    </div>
  </VirtualList>
</section>

<style>
  .row {
    display: grid;
    gap: 1rem;
    /* change the number in the repeat to control items per line */
    grid-template-columns: repeat(var(--items-per-row), minmax(0, 1fr));
  }
</style>
