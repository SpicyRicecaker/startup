<script lang="ts">
	import type { Action } from '$lib/types';

	import { onMount } from 'svelte';
	// import { invoke } from '@tauri-apps/api/tauri';
	import { tauri } from '$lib/lib';

	let actions: Action[] = [];
	onMount(async () => {
		// load config file, pref serde
		// actions = await (await invoke)('read');
		actions = tauri.invoke('hello_world');
	});

	let edit_mode: boolean = false;
</script>

{#if !edit_mode}
	{#await actions}
		loading config...
	{:then actions_awaited}
		{#each actions_awaited as action}
			{action}
			booohoo
		{/each}
	{/await}
{:else}
	edit mode
{/if}
hello
