<script lang="ts">
  import { resolve } from '$app/paths';
  import { BACKEND_URL } from '$lib/backend/common';
  import type { User } from '$lib/backend/generated-types';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { ProfilePic } from '../pfp';
  import { LogOut, Settings, User as UserIcon } from '@lucide/svelte';

  let { user }: { user: User } = $props();
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    <ProfilePic {user} class="cursor-pointer transition hover:brightness-80 active:scale-90" />
  </DropdownMenu.Trigger>
  <DropdownMenu.Content sideOffset={10} align="end">
    <DropdownMenu.Label>Account</DropdownMenu.Label>
    <DropdownMenu.Item>
      {#snippet child({ props })}
        <a {...props} href={resolve(`/user/${user.userId}/${user.handle}`)}>
          <UserIcon />Profile
        </a>
      {/snippet}
    </DropdownMenu.Item>
    <DropdownMenu.Item>
      {#snippet child({ props })}
        <a {...props} href={resolve('/settings')}>
          <Settings />Settings
        </a>
      {/snippet}
    </DropdownMenu.Item>
    <DropdownMenu.Separator />
    <DropdownMenu.Item variant="destructive">
      {#snippet child({ props })}
        <a {...props} href="{BACKEND_URL}/auth/logout">
          <LogOut />Sign out
        </a>
      {/snippet}
    </DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
