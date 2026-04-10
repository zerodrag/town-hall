<script lang="ts">
  import { Plus } from '@lucide/svelte';
  import { goto } from '$app/navigation';
  import { createQuest } from '$lib/backend/quest';
  import { Button, buttonVariants } from '$lib/components/ui/button/';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Spinner } from '$lib/components/ui/spinner';

  let dialogOpen = $state(false);
  let createQuestLoading = $state(false);
  let questTitle = $state('');
  let questError = $state('');

  const create = async (event: SubmitEvent) => {
    event.preventDefault();
    questError = '';
    createQuestLoading = true;

    const resp = await createQuest({ title: questTitle });

    if (!resp.ok) {
      questError = await resp.text();
      return;
    }

    const questId: number = await resp.json();
    await goto(`/quest/${questId}`);
    questTitle = '';
    createQuestLoading = false;
    dialogOpen = false;
  };
</script>

<form id="create-quest-form" onsubmit={create}></form>

<Dialog.Root bind:open={dialogOpen}>
  <Dialog.Trigger type="button" class={buttonVariants({ variant: 'default' })}>
    <Plus />New Quest
  </Dialog.Trigger>

  <Dialog.Content class="sm:max-w-106.25">
    <Dialog.Header>
      <Dialog.Title>New Quest</Dialog.Title>
      <Dialog.Description>Issue a new quest for Adventurers to complete!</Dialog.Description>
    </Dialog.Header>

    <div class="grid gap-4">
      <div class="grid gap-3">
        <Label for="title-1">Title</Label>
        <Input
          form="create-quest-form"
          id="title-1"
          name="title"
          bind:value={questTitle}
          placeholder="Enter quest title..."
          required
          class="invalid:border-destructive"
        />
        {#if questError}
          <p class="text-destructive">{questError}</p>
        {/if}
      </div>
    </div>

    <Dialog.Footer>
      <Dialog.Close type="button" class={buttonVariants({ variant: 'outline' })}>Cancel</Dialog.Close>

      <Button form="create-quest-form" type="submit" disabled={createQuestLoading || !questTitle.trim()}>
        {#if createQuestLoading}
          <Spinner />
        {:else}
          <Plus />
        {/if}
        Create draft
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
