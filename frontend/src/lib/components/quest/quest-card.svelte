<script lang="ts">
  import { resolve } from '$app/paths';
  import type { User } from '$lib/backend/generated-types';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import TechPill from './tech-pill.svelte';

  let {
    title,
    summary,
    techs,
    author,
    enableLink = true
  }: { title: string; summary: string; techs: string[]; author: User; enableLink: boolean } = $props();
</script>

<div class="w-4xl rounded-3xl bg-card px-4 py-3">
  <div class="flex items-center gap-2">
    <div class="text-2xl font-semibold">
      {title}
    </div>
    <div class="text-base text-foreground/50">
      by
      <a
        class="text-primary hover:text-foreground hover:underline"
        href={enableLink ? resolve(`/user/${author.userId}/${author.handle}`) : undefined}
      >
        {author.handle}
      </a>
    </div>
  </div>
  <div class="line-clamp-2 h-13 text-lg leading-tight text-foreground/80">
    {summary}
  </div>
  <div class="flex flex-wrap gap-1">
    {#each techs as tech, i (i)}
      {#if i < 6}
        <TechPill {tech} {enableLink} />
      {/if}
    {/each}
    {#if techs.length > 6}
      <Tooltip.Provider>
        <Tooltip.Root>
          <Tooltip.Trigger>
            <TechPill tech="+{techs.length - 6}" enableLink={false} />
          </Tooltip.Trigger>
          <Tooltip.Content>
            {#each techs as tech, i (i)}
              {#if i >= 6}
                <TechPill class="relative z-100" {tech} {enableLink} />
              {/if}
            {/each}
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    {/if}
  </div>
</div>
