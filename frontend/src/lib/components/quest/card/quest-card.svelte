<script lang="ts">
  import { resolve } from '$app/paths';
  import type { User } from '$lib/backend/generated-types';

  let { title, summary, techs, author }: { title: string; summary: string; techs: string[]; author: User } = $props();
</script>

<div class="w-4xl rounded-3xl bg-card px-4 py-3">
  <div class="flex items-center gap-2">
    <div class="text-xl font-semibold">
      {title}
    </div>
    <div class="text-base text-foreground/50">
      by
      <a class="hover:text-foreground hover:underline" href={resolve(`/user/${author.userId}/${author.handle}`)}>
        {author.handle}
      </a>
    </div>
  </div>
  <div class="line-clamp-2 h-13 text-base leading-snug text-foreground/70">
    {summary}
  </div>
  <div class="flex gap-1">
    {#each techs as tech (tech)}
      <span
        class="flex cursor-pointer items-center rounded-xl bg-primary/20 px-2.5 py-0.5 text-sm font-medium text-primary transition select-none active:scale-90"
      >
        {tech}
      </span>
    {/each}
  </div>
</div>
