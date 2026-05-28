<script lang="ts">
  import { resolve } from '$app/paths';
  import type { Quest } from '$lib/backend/generated-types';
  import { slugify } from '$lib/utils';
  import TechPill from '../techs/tech-pill.svelte';

  let {
    quest,
    addTech,
    clickable = false
  }: { quest: Quest; addTech?: (tech: string) => void; clickable?: boolean } = $props();
  let author = $derived(quest.poster);
</script>

<div class="grid h-32 w-4xl rounded-3xl bg-card px-4 py-3">
  {#if clickable}
    <a
      class="pointer-events-auto col-start-1 row-start-1"
      href={resolve(`/quest/${quest.questId}/${slugify(quest.title)}`)}
      aria-label={`Open quest: ${quest.title}`}
    >
    </a>
  {/if}
  <div class="pointer-events-none col-start-1 row-start-1">
    <div class="flex items-center gap-2">
      <div class="text-xl font-semibold">
        {quest.title}
      </div>
      <div class="text-base text-foreground/50">
        by
        <a
          class="pointer-events-auto hover:text-foreground hover:underline"
          href={resolve(`/user/${author.userId}/${author.handle}`)}
        >
          {author.name}
        </a>
      </div>
    </div>
    <div class="line-clamp-2 h-13 text-base leading-snug text-foreground/70">
      {quest.summary}
    </div>
    <div class="flex gap-1">
      {#each quest.techs as tech (tech)}
        <button class="pointer-events-auto" onclick={() => addTech?.(tech)}> <TechPill {tech} /></button>
      {/each}
    </div>
  </div>
</div>
