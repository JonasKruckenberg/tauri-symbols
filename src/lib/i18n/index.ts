import i18n from 'sveltekit-i18n';
import type { Config } from 'sveltekit-i18n';
import lang from './lang.json';

export const config: Config = {
	translations: {
		'en-US': { lang },
		'de-DE': { lang }
	},
	loaders: [
		{
			locale: 'en-US',
			key: 'titlebar',
			loader: async () => (await import('./en-US/titlebar.json')).default
		},
		// {
		//   locale: 'en',
		//   key: 'about',
		//   routes: ['/about'],
		//   loader: async () => (await import('./en/about.json')).default,
		// },
		// {
		//   locale: 'en',
		//   key: 'home',
		//   routes: ['/'],
		//   loader: async () => (await import('./en/home.json')).default,
		// },
		{
			locale: 'de-DE',
			key: 'titlebar',
			loader: async () => (await import('./de-DE/titlebar.json')).default
		}
		// {
		//   locale: 'cs',
		//   key: 'about',
		//   routes: ['/about'],
		//   loader: async () => (await import('./cs/about.json')).default,
		// },
		// {
		//   locale: 'cs',
		//   key: 'home',
		//   routes: ['/'],
		//   loader: async () => (await import('./cs/home.json')).default,
		// },
	]
};

export const { t, loading, locales, locale, loadTranslations } = new i18n(config);

// locale.subscribe(value => console.log(`Locale changed ${value}`))
// loading.subscribe(($loading) => $loading && console.log('Loading translations...'));
