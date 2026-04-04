<script lang="ts">
  import { navButtonStyle } from '$lib/styles/button';
  import { Info, Tags } from '@lucide/svelte';
  import { Tabs } from 'bits-ui';
  import { cn } from 'tailwind-variants';
  import QuestEditorGeneral from './quest-editor-general.svelte';
</script>

<Tabs.Root value="general" orientation="vertical" class="flex gap-6">
  <Tabs.List class="flex w-72 flex-col gap-2 self-start rounded-3xl bg-card p-3">
    <Tabs.Trigger value="general" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Info size="20" /> General
    </Tabs.Trigger>
    <Tabs.Trigger value="tags" class={cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium')}>
      <Tags />Tags
    </Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content class="flex flex-1 flex-col gap-2 rounded-3xl bg-card p-6" value="general">
    <QuestEditorGeneral />
  </Tabs.Content>
  <Tabs.Content class="flex-1 rounded-3xl bg-card p-6" value="tags">Edit tags here!</Tabs.Content>
</Tabs.Root>
