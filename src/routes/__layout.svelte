<script lang="ts">
  import "../global.css";
  import Titlebar from "../lib/Titlebar.svelte";
  import { browser } from "$app/env";

  if (browser) {
    import("@tauri-apps/api/window").then(({ getCurrent }) => {
      const win = getCurrent();
      win.show();
    });
  }

  let fontWeight = 400;
</script>

<Titlebar bind:fontWeight />
<main style={`--font-weight: ${fontWeight}`}>
  <slot />
</main>
