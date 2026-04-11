<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types.js';
  import { QuestCard } from '$lib/components/quest/card';
  import { QuestEditor } from '$lib/components/quest/editor';

  let { data } = $props();
  let quest = $derived(data.quest);

  // draft should capture initial state of quest only
  // svelte-ignore state_referenced_locally
  let draft = $state<Quest>({ ...quest });
</script>

<div class="flex flex-col gap-2">
  <div class="flex justify-center">
    <QuestCard title={draft.title} summary={draft.summary} author={data.me} />
  </div>
  <QuestEditor {quest} {draft} />
</div>
