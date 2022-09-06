import type { ServerLoad } from '@sveltejs/kit';
import { locale, loadTranslations } from '$lib/i18n';

export const load: ServerLoad = async ({ url }) => {
	const { pathname } = url;

	const defaultLocale = 'en-US'; // get from cookie / user session etc...

	const initLocale = locale.get() || defaultLocale;

	await loadTranslations(initLocale, pathname); // keep this just before the `return`

	return {};
};

export const csr = true;
export const prerender = true;
