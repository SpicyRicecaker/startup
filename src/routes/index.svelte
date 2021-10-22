<script lang="ts">
	import type { Action } from '$lib/types';
	import Grid from '$lib/grid.svelte';
	import Funky from '$lib/funky.svelte';

	import { onMount } from 'svelte';

	const tauri = (window as any).__TAURI__ as typeof import('@tauri-apps/api/tauri');

	let actions: Action[] = [];

	onMount(async () => {
		// load config file, pref serde
		actions = await tauri.invoke('load');
	});

	let edit_mode: boolean = false;

	const save = async () => {
		// update
		if (edit_mode) {
			actions = actions;
			// export to rust
			let res = await tauri.invoke('save', { actions: JSON.stringify(actions) });
		}
		edit_mode = !edit_mode;
	};

	const launch = async () => {
		// save all changes before
		actions = actions;
		let _ = await tauri.invoke('save', { actions: JSON.stringify(actions) });

		let res = await tauri.invoke('run');
		console.log(res);
	};
</script>

<div class="grid">
	<div class="control">
		<Funky on:click={save} label={edit_mode ? "save" : "edit"}/>
		{#if !edit_mode}
			<Funky on:click={launch} label="launch"/>
		{/if}
	</div>
	<div class="body">
		<Grid bind:actions bind:edit_mode />
	</div>
</div>

<style lang="scss">
	:global(#svelte, html, body) {
		width: 100%;
		height: 100%;
	}
	:global(body) {
		margin: 0;
	}

	.grid {
		width: 100%;

		display: grid;
		grid-template-rows: minmax(0, 1fr) 10fr;

		& > .control  {
			padding: 1rem;
			display: grid;
			grid-template-columns: auto 1fr;
		}
		& > .body  {
			padding: 1rem;
		}
	}
</style>
