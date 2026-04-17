<script lang="ts">
  import { resolve } from '$app/paths';
  import type { User } from '$lib/backend/generated-types';

  let { title, summary, author }: { title: string; summary: string; author: User } = $props();
</script>

<div class="h-28 w-4xl rounded-3xl bg-card p-4">
  <div class="flex items-center gap-2">
    <div class="text-xl font-medium">
      {title}
    </div>
    <div class="text-sm text-foreground/50">
      by
      <a class="hover:underline" href={resolve(`/user/${author.userId}/${author.handle}`)}>
        {author.handle}
      </a>
    </div>
  </div>
  <div class="line-clamp-2 text-lg text-foreground/70">
    {summary}
  </div>
</div>
