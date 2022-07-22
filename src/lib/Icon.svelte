<script lang="ts">
    interface Icon {
        name: string,
        symbol: string
    }

    export let icon: Icon
</script>

<div class="card">
  <i class="icon">{icon.symbol}</i>
  <span class="name">{icon.name}</span>
</div>

<style>
  .icon {
    font-size: 2.5em;
    width: 2em;
    height: 2em;
    line-height: 2em;
    margin: 0.25em;
    border: 1px solid lightgray;
    border-radius: 0.15em;
    font-weight: var(--font-weight);
  }

  .card {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .name {
    word-wrap: break-word;
    max-width: 100%;
  }
</style>
