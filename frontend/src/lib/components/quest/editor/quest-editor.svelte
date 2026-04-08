<script lang="ts">
  import { Info, Tags } from '@lucide/svelte';
  import type { Quest } from '$lib/backend/generated-types';
  import { navButtonStyle } from '$lib/styles/button';
  import { cn } from '$lib/utils';
  import { Tabs } from 'bits-ui';
  import QuestEditorGeneralTab from './quest-editor-general-tab.svelte';
  import QuestEditorUnsavedChanges from './quest-editor-unsaved-changes.svelte';

  let { quest }: { quest: Quest } = $props();

  // draft should capture initial state of quest only
  // svelte-ignore state_referenced_locally
  let draft = $state<Quest>({ ...quest });

  let editsMade = $derived(
    quest.title !== draft.title || quest.description !== draft.description || quest.status !== draft.status
  );

  const save = () => {
    // TODO: Save draft function
  };

  const reset = () => {
    draft = { ...quest };
  };

  const listStyle = 'flex flex-2 flex-col gap-2 self-start rounded-3xl bg-card p-3';
  const triggerStyle = cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium');
  const contentStyle = 'flex flex-7 flex-col gap-4 self-start rounded-3xl bg-card p-6';
</script>

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
    <QuestEditorGeneralTab bind:draft />
  </Tabs.Content>
  <Tabs.Content class={contentStyle} value="tags">Edit tags here!</Tabs.Content>
</Tabs.Root>

{#if editsMade}
  <QuestEditorUnsavedChanges {save} {reset} />
{/if}
