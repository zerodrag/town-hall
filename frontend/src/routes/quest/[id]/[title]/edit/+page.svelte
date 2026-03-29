<script lang="ts">
  import { Tabs } from 'bits-ui';
  import { navButtonStyle } from '$lib/styles/button';
  import { cn } from '$lib/utils';
  import { Info, Tags } from '@lucide/svelte';

  let { data } = $props();
  let quest = $derived(data.quest);
</script>

<h1 class="pb-6 text-4xl font-extrabold">{quest.title}</h1>

<Tabs.Root value="general" orientation="vertical" class="flex gap-6">
  <Tabs.List class="flex w-72 flex-col gap-2 rounded-3xl bg-card p-3">
    <Tabs.Trigger value="general" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Info size="20" /> General
    </Tabs.Trigger>
    <Tabs.Trigger value="tags" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Tags />Tags
    </Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="general">General Tab</Tabs.Content>
  <Tabs.Content value="tags">Edit tags here!</Tabs.Content>
</Tabs.Root>
