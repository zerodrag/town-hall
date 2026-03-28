<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Compass, LogIn, Plus } from '@lucide/svelte';
	import { SiGithub } from '@icons-pack/svelte-simple-icons';

	import { resolve } from '$app/paths';
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/state';

	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button, buttonVariants } from '$lib/components/ui/button/';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Spinner } from '$lib/components/ui/spinner';

	import { BACKEND_URL } from '$lib/backend/common';

	let { data, children } = $props();

	let dialogOpen = $state(false);
	let createQuestLoading = $state(false);
	let questTitle = $state('');
	let questError = $state('');

	const user = $derived(data.user);
	const path = $derived(page.url.pathname);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

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
								<Compass class="size-5" />Discover
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
			<form
				id="create-quest-form"
				method="POST"
				action="/quest?/new"
				use:enhance={({ formData, cancel }) => {
					const title = String(formData.get('title') ?? '')
						.trim()
						.replace(/\s+/g, ' ');

					formData.set('title', title);

					if (title.length < 10 || title.length > 100) {
						questError = 'Title must be 10-100 characters long.';
						cancel();
						return;
					}

					createQuestLoading = true;
					questError = '';

					return async ({ result }) => {
						createQuestLoading = false;

						if (result.type === 'failure') {
							questError =
								typeof result.data === 'object' &&
								result.data &&
								'error' in result.data &&
								typeof result.data.error === 'string'
									? result.data.error
									: 'Could not create quest.';
						} else if (result.type === 'redirect') {
							dialogOpen = false;
						}

						await applyAction(result);
					};
				}}
			></form>

			<Dialog.Root bind:open={dialogOpen}>
				<Dialog.Trigger type="button" class={buttonVariants({ variant: 'default' })}>
					<Plus />New Quest
				</Dialog.Trigger>

				<Dialog.Content class="sm:max-w-106.25">
					<Dialog.Header>
						<Dialog.Title>New Quest</Dialog.Title>
						<Dialog.Description>Issue a new quest for Adventurers to complete!</Dialog.Description>
					</Dialog.Header>

					<div class="grid gap-4">
						<div class="grid gap-3">
							<Label for="title-1">Title</Label>
							<Input
								form="create-quest-form"
								id="title-1"
								name="title"
								bind:value={questTitle}
								placeholder="Enter quest title..."
								required
								minlength={10}
								maxlength={100}
								class="invalid:border-destructive"
							/>
							{#if questError}
								<p class="text-sm text-destructive">{questError}</p>
							{/if}
						</div>
					</div>

					<Dialog.Footer>
						<Dialog.Close type="button" class={buttonVariants({ variant: 'outline' })}>
							Cancel
						</Dialog.Close>

						<Button
							form="create-quest-form"
							type="submit"
							disabled={createQuestLoading || !questTitle.trim()}
						>
							{#if createQuestLoading}
								<Spinner />
							{:else}
								<Plus />
							{/if}
							Create draft
						</Button>
					</Dialog.Footer>
				</Dialog.Content>
			</Dialog.Root>

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
							<a {...props} href={resolve(`/user/${user.user_id}/${user.handle}`)}>Profile</a>
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
							<a {...props} href="https://github.com/zerodrag/town-hall">
								<SiGithub />GitHub
							</a>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a {...props} href="{BACKEND_URL}/auth/logout">Sign out</a>
						{/snippet}
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<Button href="{BACKEND_URL}/auth/github">
				<LogIn />Sign in with <SiGithub /> GitHub
			</Button>
		{/if}
	</div>
</nav>
{@render children()}
