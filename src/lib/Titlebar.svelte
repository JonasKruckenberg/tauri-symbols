<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { icons } from '../stores/icons';
	import type { Icon } from '../stores/icons';
	import { fontWeight } from '../stores/font-weight';
	import { _ } from 'svelte-i18n'

	function handleSearch(pattern: string) {
		let promise: Promise<Icon[]>;
		if (pattern.trim().length == 0) {
			promise = invoke<Icon[]>('all');
		} else {
			promise = invoke<Icon[]>('search', { pattern });
		}

		promise
			.then((data) => icons.set(data))
			// this request can only fail upon invalid regex input.
			// Because entering invalid regex will inevitably happen during typing we ignore the error
			// TODO: Give some visual feedback when searching failed
			.catch((e) => console.warn(e));
	}
</script>

<header data-tauri-drag-region>
	<!-- Leading edge -->
	<div id="area-info" class="vstack">
		<span class="bold">{$_('all')} </span>
		<span>{Intl.NumberFormat().format($icons.length)} {$_('symbols')}</span>
	</div>

	<label for="search" id="area-search">
		􀊫
		<input
			id="search"
			type="search"
			autocomplete="off"
			placeholder={$_('search')}
			on:input={(event) => handleSearch(event.target.value)}
		/>
	</label>
	<label for="font-weight" id="area-font-size"
		>{$_('symbol_weight.title')}
		<select bind:value={$fontWeight} id="font-weight">
			<option value={100}>{$_('symbol_weight.thin')}</option>
			<option value={200}>{$_('symbol_weight.ultra_light')}</option>
			<option value={300}>{$_('symbol_weight.light')}</option>
			<option value={400} default>{$_('symbol_weight.regular')}</option>
			<option value={500}>{$_('symbol_weight.medium')}</option>
			<option value={600}>{$_('symbol_weight.semi_bold')}</option>
			<option value={700}>{$_('symbol_weight.bold')}</option>
			<option value={800}>{$_('symbol_weight.heavy')}</option>
			<option value={900}>{$_('symbol_weight.black')}</option>
		</select>
	</label>
</header>

<style>
	:root {
		--titlebar-background: rgba(246, 241, 249, 0.9);
	}

	header {
		width: 100vw;
		position: fixed;
		height: 52px;
		display: grid;
		grid-template-columns: 80px 1fr 1fr 1fr 1fr;
		grid-template-areas: '. info . font-size search';
		gap: 0.75em;
		align-items: center;
		transition: box-shadow 150ms, background-color 150ms;
		z-index: 100;
		box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.13);
		background-color: var(--titlebar-background);
		-webkit-backdrop-filter: blur(20px);
		backdrop-filter: blur(10px);
		font-size: 0.9rem;
		line-height: 1.1rem;
		padding: 0 10px;
		box-sizing: border-box;
	}

	#area-info {
		grid-area: info;
	}
	#area-search {
		grid-area: search;
	}
	#area-font-size {
		grid-area: font-size;
	}

	.bold {
		font-weight: bold;
	}

	.vstack {
		display: flex;
		flex-direction: column;
	}

	header > * {
		user-select: none;
		pointer-events: none;
	}

	select,
	input {
		pointer-events: all;
	}

	select {
		font-size: 1.2em;
	}

	select:focus {
		outline: none;
	}

	label[for='search'] {
		border: 1px solid var(--border-color);
		padding: 0 0.5em;
		border-radius: 7px;
		box-sizing: border-box;
		display: flex;
		align-items: center;
		color: rgb(115, 115, 115);
	}

	input[type='search'] {
		font-size: 0.8rem;
		background-color: transparent;
		border: none;
	}

	input[type='search']::placeholder {
		font-weight: 500;
	}

	input[type='search']:focus {
		outline: none;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			--titlebar-background: rgba(55, 51, 58, 0.9);
		}

		header {
			box-shadow: 0px 1px 1px 0px rgba(0, 0, 0, 0.2);
		}
	}
</style>
