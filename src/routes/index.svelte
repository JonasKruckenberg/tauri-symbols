<script lang="ts">
  import Icon from "../lib/Icon.svelte";
  import symbols from "../assets/data.json";
  import { onDestroy, onMount } from "svelte";

  let lazyIconObserver: IntersectionObserver;

  onMount(() => {
    const lazyIcons = [].slice.call(document.querySelectorAll(".icon.lazy"));

    lazyIconObserver = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          let lazyIcon = entry.target;
          lazyIcon.classList.remove("lazy");
          lazyIcon.children[0].textContent = lazyIcon.dataset.symbol
          lazyIconObserver.unobserve(lazyIcon);
        }
      });
    }, { rootMargin: '800px' });

    lazyIcons.forEach((lazyIcon) => {
      lazyIconObserver.observe(lazyIcon);
    });
  });

  onDestroy(() => {
    if (lazyIconObserver) lazyIconObserver.disconnect();
  });

  const aboveFold = 48
</script>

<section id="root">
  {#each symbols as symbol, i}
    <Icon icon={symbol} lazy={i > aboveFold}/>
  {/each}
</section>

<style>
  section {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    grid-auto-rows: minmax(min-content, 9em);
    height: 100vh;
    overflow-y: scroll;
    padding: 0 3em;
    padding-top: 48px;
  }
</style>
