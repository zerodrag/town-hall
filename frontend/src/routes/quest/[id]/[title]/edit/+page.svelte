<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types.js';
  import { QuestEditor } from '$lib/components/quest/editor';
  import QuestCard from '$lib/components/quest/card/quest-card.svelte';

  let { data } = $props();
  let quest = $derived(data.quest);

  // draft should capture initial state of quest only
  // svelte-ignore state_referenced_locally
  let draft = $state<Quest>({ ...quest });
</script>

<div class="flex flex-col gap-2">
  <div class="flex justify-center">
    <QuestCard quest={draft} />
  </div>
  <QuestEditor {quest} {draft} />
</div>