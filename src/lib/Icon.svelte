<script lang="ts">
  interface Icon {
    name: string;
    symbol: string;
  }

  export let icon: Icon;
  export let lazy: boolean
</script>

<div class={`icon ${lazy && 'lazy'}`} data-symbol={icon.symbol}>
  <i>{!lazy ? icon.symbol: ' '}</i>
  <span class="name">{icon.name}</span>
</div>

<style>
  .icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
    /* height: 12rem; */
    /* max-height: 12rem; */
  }

  .icon > i {
    font-size: 2.5em;
    width: 2em;
    height: 2em;
    line-height: 2em;
    margin: 0.25em;
    border: 1px solid lightgray;
    border-radius: 0.15em;
    font-weight: var(--font-weight);
  }

  .name {
    word-wrap: break-word;
    max-width: 100%;
    max-height: 100%;
    text-overflow: ellipsis;
  }
</style>
