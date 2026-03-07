<script lang="ts">
	import type { User, CreateUserRequest } from '$lib/generated';

	let user: User | null = null;

	async function addUser() {
		const request: CreateUserRequest = {
			githubId: '62168687',
			username: 'drag0n1zed'
		};

		const response = await fetch('/api/create-user', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(request)
		});

		user = (await response.json()) as User;
	}

	import { Button } from '$lib/components/ui/button/index';
</script>

<Button onclick={addUser}>Add user</Button>

{#if user}
	<p>Success! Your user id is: {user.id}</p>
{/if}
