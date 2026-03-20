<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { resolve } from '$app/paths';
	import { ModeWatcher } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	let { data, children } = $props();
	let user = $derived(data.user);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<ModeWatcher />
<nav class="flex items-center justify-between px-4 py-3">
	<div>
		<a href={resolve('/')}>Town Hall</a>
	</div>
	<div class="flex items-center gap-3">
		{#if user}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					<Avatar.Root class="size-9 hover:brightness-90 active:translate-y-px">
						<Avatar.Image
							src="https://avatars.githubusercontent.com/u/{user.github_id}"
							alt="@{user.handle}"
						/>
						<Avatar.Fallback>{user.handle.slice(0, 2).toUpperCase()}</Avatar.Fallback>
					</Avatar.Root>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content>
					<DropdownMenu.Item>
						{#snippet child({ props }: { props: Record<string, unknown> })}
							<a href="http://localhost:3000/auth/logout" {...props}>Logout</a>
						{/snippet}
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<Button href="http://localhost:3000/auth/github">Log in</Button>
		{/if}
	</div>
</nav>
{@render children()}
