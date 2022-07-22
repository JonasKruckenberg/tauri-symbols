<script lang="ts">
  import Interactor from "./Intersector.svelte";
  interface Icon {
    name: string;
    symbol: string;
  }

  export let icon: Icon;
</script>

<Interactor once={true} let:intersecting>
  {#if intersecting}
    <div class="icon" data-symbol={icon.symbol}>
      <i>{icon.symbol}</i>
      <span class="name">{icon.name}</span>
    </div>
  {/if}
</Interactor>

<style>
  .icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
  }

  .icon > i {
    font-size: 2.5em;
    width: 2em;
    height: 2em;
    line-height: 2em;
    border: 1px solid lightgray;
    border-radius: 0.15em;
    font-weight: var(--font-weight);
  }

  .name {
    white-space: nowrap;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
  }
</style>
