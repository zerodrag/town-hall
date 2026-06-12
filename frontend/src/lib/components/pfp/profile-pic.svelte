<script lang="ts">
  import type { User } from '$lib/backend/generated-types';
  import * as Avatar from '$lib/components/ui/avatar';

  let { user, class: className }: { user: User; class?: string } = $props();
</script>

<Avatar.Root class={className}>
  <Avatar.Image src={`https://avatars.githubusercontent.com/u/${user.githubId}`} alt="@{user.handle}" />
  <Avatar.Fallback>{user.handle.slice(0, 2).toUpperCase()}</Avatar.Fallback>
</Avatar.Root>
