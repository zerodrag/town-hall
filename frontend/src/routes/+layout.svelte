<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { data, children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav>
	{#if data.user}
		<span>Hello, {data.user.handle}</span>
		<a href="http://localhost:3000/auth/logout">Log out</a>
	{:else}
		<a href="http://localhost:3000/auth/github">Log in</a>
	{/if}
</nav>
{@render children()}
