<script lang="ts">
	import { onMount } from 'svelte';

    export let label: string = "";

	let inMotion: boolean = false;

	onMount(() => {
		shuffleClip();
	});

	const shuffleClip = () => {
		// Generate 8 random numbers between 0 and 10
		// let out = [];
		for (let i = 1; i <= 8; i++) {
			// generate random number
			let num = Math.random() * 10;
			buttonBinding.style.setProperty(`--cl-offset-${i}`, `${num}%`);
		}
	};

	let buttonBinding: HTMLButtonElement;

	function shuffle() {
		if (inMotion) {
			shuffleClip();
			window.setTimeout(shuffle, 300);
		}
	}

	const playback = () => {
		if (inMotion) {
			inMotion = false;
		} else {
			inMotion = true;
			shuffle();
		}
	};
</script>

<!-- on:click means forward all / propogate click events upwards -->
<button id="button" on:click bind:this={buttonBinding} on:mouseenter={playback} on:mouseleave={playback}
    >{label}</button
>

<style lang="scss">
    // TODO if we have a shifting white background behind black it looks 10x better :)
	#button {
		transition: 0.3s;
		font-size: 1.5rem;
		font-family: 'Times New Roman', Times, serif;
		// must be absolutely positioned
		color: white;
		background-color: black;
        opacity: 70%;
		border: none;
		outline: none;
		&:hover {
			cursor: pointer;
		}

        padding: 2rem;

        &:active {
            background-color: white;
        }

		// starting from the top left corner, clockwise
		// clip-path: polygon(0% 0%, 100% 0%, 100% 100%, 0 100%);
		// clip-path: polygon(10% 10%, 90% 10%, 90% 90%, 10% 90%);
		// each of the values are from 0-10, so we can just straightup insert our offset
		// 0-10, 0-10
		// 90-100, 0-10
		// 90-100, 90-100
		// 0-10, 90-100
		// except we're adding calc(100-) in various places so we can just generate 8 numbers from 0-10 and mix those around
		clip-path: polygon(
			var(--cl-offset-1, 0) var(--cl-offset-2, 0),
			calc(100% - var(--cl-offset-3, 0)) var(--cl-offset-4, 0),
			calc(100% - var(--cl-offset-5, 0)) calc(100% - var(--cl-offset-6, 0)),
			var(--cl-offset-7, 0) calc(100% - var(--cl-offset-8, 0))
		);
	}
</style>
