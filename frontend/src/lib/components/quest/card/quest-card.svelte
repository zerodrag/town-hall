<script lang="ts">
  import { resolve } from '$app/paths';
  import type { Quest, User } from '$lib/backend/generated-types';

  let { quest, author = undefined }: { quest: Quest; author?: User } = $props();
</script>

<div class="w-4xl rounded-3xl bg-card p-4">
  <div class="flex items-center gap-2">
    <div class="text-xl font-medium">
      {quest.title}
    </div>
    {#if author}
      <div class="text-sm text-foreground/50">
        by
        <a class="hover:underline" href={resolve(`/user/${author.userId}/${author.handle}`)}>
          {author.handle}
        </a>
      </div>
    {/if}
  </div>
  <div class="line-clamp-2 text-lg text-foreground/70">
    {quest.description}
  </div>
</div>
