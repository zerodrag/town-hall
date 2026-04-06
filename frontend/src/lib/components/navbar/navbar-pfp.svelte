<script lang="ts">
  import { resolve } from '$app/paths';
  import { BACKEND_URL } from '$lib/backend/common';
  import type { User } from '$lib/backend/generated-types';
  import * as Avatar from '$lib/components/ui/avatar';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { SiGithub } from '@icons-pack/svelte-simple-icons';

  let { user }: { user: User } = $props();
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    <Avatar.Root class="cursor-pointer transition-all hover:brightness-80 active:scale-90">
      <Avatar.Image src="https://avatars.githubusercontent.com/u/{user.githubId}" alt="@{user.handle}" />
      <Avatar.Fallback>{user.handle.slice(0, 2).toUpperCase()}</Avatar.Fallback>
    </Avatar.Root>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content sideOffset={10} align="end">
    <DropdownMenu.Label>Account</DropdownMenu.Label>
    <DropdownMenu.Item>
      {#snippet child({ props })}
        <a {...props} href={resolve(`/user/${user.userId}/${user.handle}`)}>Profile</a>
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
    <DropdownMenu.Item variant="destructive">
      {#snippet child({ props })}
        <a {...props} href="{BACKEND_URL}/auth/logout">Sign out</a>
      {/snippet}
    </DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
