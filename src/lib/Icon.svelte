<script lang="ts">
  import Interactor from "./Intersector.svelte";
  import type {Icon} from '../stores/icons'

  export let icon: Icon;
</script>

<Interactor once={true} let:intersecting>
  {#if intersecting}
    <div class="icon" data-symbol={icon[1]}>
      <i>{icon[1]}</i>
      <span class="name">{icon[0]}</span>
    </div>
  {/if}
</Interactor>

<style>
  .icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
    text-align: center;
  }

  .icon > i {
    font-size: 2.5em;
    width: 2em;
    height: 2em;
    line-height: 2em;
    border: 1px solid var(--border-color);
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

  @media (prefers-color-scheme: dark) {
    .icon > i {
      border: 1px solid var(--border-color);
    }
  }
</style>
