<script lang="ts" context="module">
	import '../global.css';
	import Titlebar from '../lib/Titlebar.svelte';
	import { browser } from '$app/environment';
	import { fontWeight } from '../stores/font-weight';
	import { locale } from '$lib/i18n';
	import * as Sentry from '@timfish/tauri-sentry'
	import { BrowserTracing } from '@sentry/tracing';

	if (browser) {
		Sentry.init({
			integrations: [new BrowserTracing()],

			// Set tracesSampleRate to 1.0 to capture 100%
			// of transactions for performance monitoring.
			// We recommend adjusting this value in production
			tracesSampleRate: 1.0,
		});

		import('@tauri-apps/api/window').then(({ getCurrent }) => {
			const win = getCurrent();
			win.show();
		});

		fontWeight.subscribe((val) => {
			document.documentElement.style.setProperty('--font-weight', val.toString());
		});

		locale.set(navigator.language);
	}
</script>

<Titlebar />
<main>
	<slot />
</main>
