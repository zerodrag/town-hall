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

  let { quest, draft = $bindable() }: { quest: Quest; draft: Quest } = $props();


  const calculateDeltaDraft = (draft: Quest, quest: Quest) => {
    const delta: UpdateQuestRequest = {};
    if (quest.title !== draft.title) delta.title = draft.title;
    if (quest.summary !== draft.summary) delta.summary = draft.summary;
    if (quest.details !== draft.details) delta.details = draft.details;
    if (quest.status !== draft.status) delta.status = draft.status;
    return delta;
  };
  let draftDelta = $derived(calculateDeltaDraft(draft, quest));
  let editsMade = $derived(Object.keys(draftDelta).length !== 0);

  let saveError = $state('');
  let savingIcon: 'pending' | 'loading' | 'success' = $state('pending');

  const update = async (event: SubmitEvent) => {
    event.preventDefault();
    saveError = '';
    savingIcon = 'loading';

    if (!editsMade) return;

    const response = await updateQuest(quest.questId, draftDelta);

    if (!response.ok) {
      saveError = await response.text();
      savingIcon = 'pending';
      return;
    }

    savingIcon = 'success';
    setTimeout(async () => {
      await invalidate(`${BACKEND_URL}/quests/${quest.questId}`);
      savingIcon = 'pending';
    }, 200);
  };

  const reset = () => {
    saveError = '';
    draft = { ...quest };
  };

  const updateFormId = 'update-quest-form';

  const listStyle = 'flex flex-2 flex-col gap-2 self-start rounded-3xl bg-card p-3';
  const triggerStyle = cn(navButtonStyle(), 'flex h-10 gap-2 px-4 text-lg font-medium');
  const contentStyle = 'flex flex-7 flex-col gap-4 self-start rounded-3xl bg-card p-6';
</script>

<form id={updateFormId} onsubmit={update}></form>

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
  <QuestEditorUnsavedChanges {updateFormId} {reset} {saveError} {savingIcon} />
{/if}
