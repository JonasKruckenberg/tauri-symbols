<script lang="ts">
	import '../global.css';
	import Titlebar from '../lib/Titlebar.svelte';
	import { browser } from '$app/environment';
	import { fontWeight } from '../stores/font-weight';
	import { locale } from '$lib/i18n';

	if (browser) {
		import('@tauri-apps/api/window').then(({ getCurrent }) => {
			const win = getCurrent();
			win.show();
		});

		fontWeight.subscribe((val) => {
			document.documentElement.style.setProperty('--font-weight', val.toString());
		});

		locale.set(navigator.language)
	}
</script>

<Titlebar />
<main>
	<slot />
</main>
