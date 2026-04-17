<script lang="ts">
  import { IdCard, Tags, TextAlignStart } from '@lucide/svelte';
  import { invalidate } from '$app/navigation';
  import { BACKEND_URL } from '$lib/backend/common';
  import type { Quest, UpdateQuestRequest } from '$lib/backend/generated-types';
  import { updateQuest } from '$lib/backend/quest';
  import { navButtonStyle } from '$lib/styles/button';
  import { cn } from '$lib/utils';
  import { Tabs } from 'bits-ui';
  import QuestEditorCardTab from './quest-editor-card-tab.svelte';
  import QuestEditorDetailsTab from './quest-editor-details-tab.svelte';
  import QuestEditorTechsTab from './quest-editor-techs-tab.svelte';
  import QuestEditorUnsavedChanges from './quest-editor-unsaved-changes.svelte';

  let { quest, draft = $bindable() }: { quest: Quest; draft: Quest } = $props();

  const calculateDeltaDraft = (draft: Quest, quest: Quest) => {
    const delta: UpdateQuestRequest = {};
    if (quest.title !== draft.title) delta.title = draft.title;
    if (quest.summary !== draft.summary) delta.summary = draft.summary;
    if (quest.details !== draft.details) delta.details = draft.details;
    if (!(quest.techs.length === draft.techs.length && quest.techs.every((val, index) => val === draft.techs[index]))) {
      delta.techs = draft.techs;
    }
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

<Tabs.Root value="card" orientation="vertical" class="flex gap-6">
  <Tabs.List class={listStyle}>
    <Tabs.Trigger value="card" class={triggerStyle}>
      <IdCard size={20} /> Card
    </Tabs.Trigger>
    <Tabs.Trigger value="details" class={triggerStyle}>
      <TextAlignStart size={20} /> Details
    </Tabs.Trigger>
    <Tabs.Trigger value="techs" class={triggerStyle}>
      <Tags size={20} /> Techs
    </Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content class={contentStyle} value="card">
    <QuestEditorCardTab bind:draft />
  </Tabs.Content>
  <Tabs.Content class={contentStyle} value="details">
    <QuestEditorDetailsTab bind:draft />
  </Tabs.Content>
  <Tabs.Content class={contentStyle} value="techs">
    <QuestEditorTechsTab bind:draft />
  </Tabs.Content>
</Tabs.Root>

{#if editsMade}
  <QuestEditorUnsavedChanges {updateFormId} {reset} {saveError} {savingIcon} />
{/if}

<div class="h-10"></div>
