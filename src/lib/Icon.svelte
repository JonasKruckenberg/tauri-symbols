<script lang="ts">
  import type { Icon } from "../stores/icons";

  export let icon: Icon;
  export let lazy: boolean;
</script>

<div class={`icon ${lazy && "lazy"}`} data-symbol={icon[1]}>
  <i>{!lazy ? icon[1] : " "}</i>
  <span class="name">{icon[0]}</span>
</div>

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
