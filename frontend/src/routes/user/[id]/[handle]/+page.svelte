<script lang="ts">
  import { ProfilePic } from '$lib/components/pfp';
  import { Separator } from '$lib/components/ui/separator';

  const { data } = $props();
  const user = $derived(data.targetUser);
</script>

<div class="mb-2 flex">
  <ProfilePic {user} class="h-12 w-12" />
  <div class="ml-4 flex flex-col justify-center">
    <!-- name + handle -->
    <div class="flex gap-3 text-2xl">
      {user.name}
      <a
        rel="external noreferrer noopener"
        target="_blank"
        href="https://github.com/{user.handle}"
        class="text-foreground/50 hover:underline"
      >
        @{user.handle}
      </a>
    </div>
  </div>
</div>
<Separator />
