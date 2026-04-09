<script lang="ts">
  import { Info, Tags } from '@lucide/svelte';
  import { invalidate } from '$app/navigation';
  import { BACKEND_URL } from '$lib/backend/common';
  import type { Quest, UpdateQuestRequest } from '$lib/backend/generated-types';
  import { updateQuest } from '$lib/backend/quest';
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

  let saveError = $state('');
  let savingIcon: 'pending' | 'loading' | 'success' = $state('pending');

  const update = async (event: SubmitEvent) => {
    event.preventDefault();
    saveError = '';
    savingIcon = 'loading';

    const delta: UpdateQuestRequest = {};
    if (quest.title !== draft.title) delta.title = draft.title;
    if (quest.description !== draft.description) delta.description = draft.description;
    if (quest.status !== draft.status) delta.status = draft.status;

    if (Object.keys(delta).length === 0) return;

    const response = await updateQuest(quest.questId, delta);

    if (!response.ok) {
      saveError = await response.text();
      savingIcon = 'pending';
      return;
    }

    savingIcon = 'success';
    setTimeout(async () => {
      await invalidate(`${BACKEND_URL}/quests/${quest.questId}`);
      savingIcon = 'pending';
    }, 500);
  };

  const reset = () => {
    saveError = '';
    draft = { ...quest };
  };

  const listStyle = 'flex flex-2 flex-col gap-2 self-start rounded-3xl bg-card p-3';
  const triggerStyle = cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium');
  const contentStyle = 'flex flex-7 flex-col gap-4 self-start rounded-3xl bg-card p-6';
</script>

<form id="update-quest-form" onsubmit={update}></form>

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
  <QuestEditorUnsavedChanges {reset} {saveError} {savingIcon} />
{/if}
