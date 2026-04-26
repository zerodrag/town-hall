<script lang="ts">
  import { invalidate } from '$app/navigation';
  import { BACKEND_URL } from '$lib/backend/common';
  import type { Quest } from '$lib/backend/generated-types';
  import { updateQuest } from '$lib/backend/quest';
  import { Button } from '$lib/components/ui/button';

  let { quest, editsMade }: { quest: Quest; editsMade: boolean } = $props();

  const publish = async () => {
    const response = await updateQuest(quest.questId, { status: 'ongoing' });
    if (response.ok) {
      await invalidate(`${BACKEND_URL}/quests/${quest.questId}`);
    }
  };
</script>

{#if quest.status === 'draft'}
  <h1 class="text-2xl font-semibold">Status</h1>
  <Button disabled={editsMade} onclick={publish}>
    {#if editsMade}
      Save changes before publishing
    {:else}
      Publish Quest
    {/if}
  </Button>
  {#if editsMade}
    <p class="text-sm text-muted-foreground">Save your changes before publishing.</p>
  {:else}
    <p class="text-sm text-muted-foreground">
      Publishing makes your quest visible to all users. This action cannot be undone.
    </p>
  {/if}
{:else}
  <h1 class="text-2xl font-semibold">Status</h1>
  <p class="text-muted-foreground">This quest has been published and is now visible to all users.</p>
{/if}
