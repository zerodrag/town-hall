<script lang="ts">
  import { resolve } from '$app/paths';
  import type { Quest } from '$lib/backend/generated-types';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import { cn, slugify } from '$lib/utils';
  import TechPill from '../techs/tech-pill.svelte';

  let {
    class: className,
    quest,
    addTech,
    clickable = false
  }: { class?: string; quest: Quest; addTech?: (tech: string) => void; clickable?: boolean } = $props();
  let author = $derived(quest.poster);
</script>

<div
  class={cn(
    className,
    'group/card grid min-h-32 rounded-3xl border bg-card px-4 py-3 transition has-[>a:active]:scale-95 has-[>a:hover]:brightness-120'
  )}
>
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
      <div class="text-xl font-semibold group-has-[>a:hover]/card:underline">
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
    <div class="flex flex-wrap gap-1">
      {#each quest.techs.slice(0, 5) as tech (tech)}
        <button class="pointer-events-auto" onclick={() => addTech?.(tech)}> <TechPill {tech} /></button>
      {/each}
      {#if quest.techs.length > 5}
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger>
              <TechPill class="pointer-events-auto" tech={`+${quest.techs.length - 5}`} />
            </Tooltip.Trigger>
            <Tooltip.Content class="border bg-background shadow-2xl" arrowClasses="bg-background shadow-2xl">
              {#each quest.techs.slice(5) as tech (tech)}
                <button class="pointer-events-auto" onclick={() => addTech?.(tech)}>
                  <TechPill class="bg-muted" {tech} /></button
                >
              {/each}
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
      {/if}
    </div>
  </div>
</div>
