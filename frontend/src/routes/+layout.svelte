<script lang="ts">
	import type { LayoutData } from './$types';
	import { onMount } from 'svelte';
	import { session } from '$lib/session';
	import { goto } from '$app/navigation';

	export let data: LayoutData;

	$: loading = $session.loading ?? true;

	onMount(async () => {
		const user = await data.getAuthUser();

		const loggedIn = user !== null;

		session.update((cur) => {
			loading = false;
			return {
				...cur,
				user,
				loggedIn,
				loading: false
			};
		});

		if (loggedIn) {
			await goto('/protected');
		} else {
			await goto('/');
		}
	});
</script>

{#if loading}
	<div>Loading...</div>
{:else}
	<slot />
{/if}
