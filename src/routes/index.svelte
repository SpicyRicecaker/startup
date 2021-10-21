<script lang="ts">
	import type { Action } from '$lib/types';

	import { onMount } from 'svelte';
	import { new_action } from '$lib/lib';
	import { tauri } from '$lib/lib';

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
</script>

<button on:click={save}>
	{#if edit_mode}
		save
	{:else}
		edit
	{/if}
</button>

{#if edit_mode}
	<button on:click={() => (actions = [...actions, new_action()])}>+</button>
	<div class="grid">
		{#each actions as action, i}
			<div>
				<div>
					<label for="name">n:</label>
					<input bind:value={action.name} id="name" type="text" />
				</div>
				<div>
					<label for="command"><code>>:</code></label>
					<input bind:value={action.props.command} id="command" type="text" />
				</div>
				<button
					on:click={() => {
						actions = [...actions.slice(0, i), ...actions.slice(i + 1)];
					}}>x</button
				>
			</div>
		{/each}
	</div>
{:else}
	<button>launch</button>
	<div class="grid">
		{#each actions as action, i}
			<div>
				<div class="name">{action.name}</div>
				<code>{action.props.command}</code>
				<div>
					<input bind:checked={action.run} type="checkbox" />
				</div>
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		grid-auto-rows: minmax(100px, auto);
	}
	.name {
		font-size: 1.2rem;
	}
</style>
