<script lang="ts">
  import { QuestCard } from '$lib/components/quest/card';
  import { QuestEditor } from '$lib/components/quest/editor';

  let { data } = $props();
  let quest = $derived(data.quest);
</script>

<div class="flex flex-col gap-2">
  <div class="flex justify-center">
    <QuestCard {quest} author={data.me} />
  </div>
  <QuestEditor {quest} />
</div>
