<script lang="ts">
  import { navButtonStyle } from '$lib/styles/button';
  import { Info, Tags } from '@lucide/svelte';
  import { Tabs } from 'bits-ui';
  import QuestEditorGeneral from './quest-editor-general.svelte';
  import type { Quest } from '$lib/backend/generated-types';
  import { cn } from '$lib/utils';

  let { quest }: { quest: Quest } = $props();

  // draft is meant to use a snapshot of the quest
  // svelte-ignore state_referenced_locally
  let draft = $state<Quest>({
    ...quest
  });

  let editsMade = $derived(
    quest.title !== draft.title || quest.description !== draft.description || quest.status !== draft.status
  );

  let listStyle = 'flex flex-2 flex-col gap-2 self-start rounded-3xl bg-card p-3';
  let triggerStyle = cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium');
  let contentStyle = 'flex flex-7 flex-col gap-4 self-start rounded-3xl bg-card p-6';
</script>

{#if editsMade}
  UNSAVED CHANGES MADE!!!!
{/if}

<Tabs.Root value="general" orientation="vertical" class="flex gap-6">
  <Tabs.List class={listStyle}>
    <Tabs.Trigger value="general" class={triggerStyle}>
      <Info size={20} /> General
    </Tabs.Trigger>
    <Tabs.Trigger value="tags" class={triggerStyle}>
      <Tags size={20} /> Tags
    </Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content class={contentStyle} value="general">
    <QuestEditorGeneral bind:draft />
  </Tabs.Content>
  <Tabs.Content class={contentStyle} value="tags">Edit tags here!</Tabs.Content>
</Tabs.Root>
