<script lang="ts">
  import { resolve } from '$app/paths';
  import type { Quest } from '$lib/backend/generated-types';
  import TechPill from '../tech-pill.svelte';

  let { quest }: { quest: Quest } = $props();
  let author = $derived(quest.poster);
</script>

<div class="w-4xl rounded-3xl bg-card px-4 py-3">
  <div class="flex items-center gap-2">
    <div class="text-xl font-semibold">
      {quest.title}
    </div>
    <div class="text-base text-foreground/50">
      by
      <a class="hover:text-foreground hover:underline" href={resolve(`/user/${author.userId}/${author.handle}`)}>
        {author.handle}
      </a>
    </div>
  </div>
  <div class="line-clamp-2 h-13 text-base leading-snug text-foreground/70">
    {quest.summary}
  </div>
  <div class="flex gap-1">
    {#each quest.techs as tech (tech)}
      <TechPill {tech} />
    {/each}
  </div>
</div>