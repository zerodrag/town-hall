<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Compass } from '@lucide/svelte';

	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ModeWatcher } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu';

	let { data, children } = $props();
	const user = $derived(data.user);
	const path = $derived(page.url.pathname);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<ModeWatcher />
<nav class="mx-auto flex w-full max-w-7xl items-center justify-between px-6 py-6">
	<div class="flex-1 text-2xl font-bold">
		<a href={resolve('/')}>Town Hall</a>
	</div>
	<div class="flex-none">
		<NavigationMenu.Root>
			<NavigationMenu.List>
				<NavigationMenu.Item>
					<NavigationMenu.Link active={path === '/discover'}>
						{#snippet child({ props })}
							<a {...props} href={resolve('/discover')}>
								<Compass class="size-5" />
								<span>Discover</span>
							</a>
						{/snippet}
					</NavigationMenu.Link>
				</NavigationMenu.Item>
				<NavigationMenu.Item>
					<NavigationMenu.Link active={path === '/user'}>
						{#snippet child({ props })}
							<a {...props} href={resolve('/user')}>User</a>
						{/snippet}
					</NavigationMenu.Link>
				</NavigationMenu.Item>
			</NavigationMenu.List>
		</NavigationMenu.Root>
	</div>
	<div class="flex flex-1 items-center justify-end gap-4">
		{#if user}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					<Avatar.Root
						class="cursor-pointer transition-transform hover:brightness-90 active:scale-90"
					>
						<Avatar.Image
							src="https://avatars.githubusercontent.com/u/{user.github_id}"
							alt="@{user.handle}"
						/>
						<Avatar.Fallback>{user.handle.slice(0, 2).toUpperCase()}</Avatar.Fallback>
					</Avatar.Root>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Label>Account</DropdownMenu.Label>
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a {...props} href={resolve(`/user/${user.handle}`)}>Profile</a>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a {...props} href={resolve('/settings')}>Settings</a>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Label>Links</DropdownMenu.Label>
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a {...props} href="https://github.com/zerodrag/town-hall">GitHub</a>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a {...props} href="http://localhost:3000/auth/logout">Log out</a>
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
