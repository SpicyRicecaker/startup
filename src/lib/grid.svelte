<script lang="ts">
	import { Action, ExecType } from './types';
	import { new_action } from '$lib/lib';
	import Funky from './funky.svelte';

	export let actions: Action[];
	export let edit_mode: boolean;

	const on_drag_start = (ev: DragEvent) => {
		ev.dataTransfer.setData('text/plain', (ev.target as HTMLElement).innerHTML);
	}
</script>

{#if edit_mode}
	<div class="grid">
		{#each actions as action, i}
			<div class="edit" on:dragstart={(e) => on_drag_start(e)}>
				<div>
					<label for="name">n:</label>
					<input bind:value={action.name} id="name" type="text" />
				</div>
				<div>
					<label for="command"><code>>:</code></label>
					<input bind:value={action.config.program} id="command" type="text" />
				</div>
				<div>
					<span>
						<label for="output">Output</label>
						<input
							bind:group={action.config.exec_type}
							value={ExecType.Output}
							id="output"
							type="radio"
						/>
					</span>
					<span>
						<label for="spawn">Spawn</label>
						<input
							bind:group={action.config.exec_type}
							value={ExecType.Spawn}
							id="spawn"
							type="radio"
						/>
					</span>
				</div>
				<button
					on:click={() => {
						actions = [...actions.slice(0, i), ...actions.slice(i + 1)];
					}}>x</button
				>
			</div>
		{/each}
		<Funky on:click={() => (actions = [...actions, new_action()])} label="+" />
	</div>
{:else}
	<div class="grid">
		{#each actions as action, i}
			<div
				class="display"
				on:click={() => {
					action.run = !action.run;
				}}
			>
				<div class="name">{action.name}</div>
				<pre>
					<code>{action.config.program}</code>
				</pre>
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
		grid-template-columns: repeat(3, minmax(0, 1fr));
		grid-auto-rows: minmax(100px, auto);
		gap: 1rem;
		// nonstandard
		user-select: none;
		& > .display {
			&:hover {
				cursor: pointer;
			}
			& > pre {
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
				opacity: 50%;
			}
		}
		& > .edit {
			&:hover {
				cursor: pointer;
			}
		}
	}
	.name {
		font-size: 1.2rem;
	}
</style>
