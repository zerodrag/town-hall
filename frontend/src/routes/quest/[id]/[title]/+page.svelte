<script lang="ts">
  import { SquarePen } from '@lucide/svelte';
  import { resolve } from '$app/paths';
  import { ProfilePic } from '$lib/components/pfp';
  import MarkdownRenderer from '$lib/components/quest/markdown/markdown-renderer.svelte';
  import { TechPill } from '$lib/components/quest/techs';
  import { Button } from '$lib/components/ui/button';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { slugify } from '$lib/utils';

  const { data } = $props();
  let quest = $derived(data.quest);
  let me_owns_quest = $derived(data.me?.userId === quest.poster.userId);
</script>

<div class="flex flex-col gap-3">
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-1">
      <!-- Title -->
      <h1 class="text-3xl font-bold">{quest.title}</h1>
      <!-- Summary -->
      <p class="mb-1 max-w-5xl text-base/5">{quest.summary}</p>
      <div class="mb-1 flex gap-2">
        {#each quest.techs as tech (tech)}
          <a href={resolve(`/discover?techs=${tech}`)}>
            <TechPill {tech} />
          </a>
        {/each}
      </div>
    </div>
    {#if me_owns_quest}
      <Button href={resolve(`/quest/${quest.questId}/${slugify(quest.title)}/edit`)}>
        <SquarePen />Edit quest
      </Button>
    {/if}
  </div>
  <Separator />
  <div class="flex gap-3">
    <!-- Details -->
    <MarkdownRenderer markdown={quest.details} class="max-w-none flex-3 rounded-xl bg-card p-8" />
    <!-- Sidebar -->
    <div class="flex-1">
      <!-- Author -->
      <div class="flex flex-col gap-2 rounded-xl bg-card p-4">
        <div class="text-lg font-bold">Author</div>
        <a
          class="group flex items-center gap-2 transition active:scale-95"
          href={resolve(`/user/${quest.poster.userId}/${slugify(quest.poster.handle)}`)}
        >
          <ProfilePic class="h-6 w-6 " user={quest.poster} />
          <div class="transition group-hover:underline">{quest.poster.name}</div>
          <div class="flex text-foreground/50">
            @{quest.poster.handle}
          </div>
        </a>
      </div>
    </div>
  </div>
</div>
