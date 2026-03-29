<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { Links, CreateQuest, PfpMenu, GithubLogin, Logo } from '$lib/components/navigation';

	let { data, children } = $props();

	const user = $derived(data.user);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="mx-auto flex w-full max-w-7xl items-center justify-between px-6 py-6">
	<div class="flex-1">
		<Logo />
	</div>
	<div class="flex-none">
		<Links />
	</div>

	<div class="flex flex-1 items-center justify-end gap-4">
		{#if user}
			<CreateQuest />
			<PfpMenu {user} />
		{:else}
			<GithubLogin />
		{/if}
	</div>
</nav>
{@render children()}
