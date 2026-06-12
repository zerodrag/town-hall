<script lang="ts">
  import { ChevronLeft, ChevronRight } from '@lucide/svelte';
  import { page } from '$app/state';
  import { ProfilePic } from '$lib/components/pfp';
  import QuestCard from '$lib/components/quest/card/quest-card.svelte';
  import { Button } from '$lib/components/ui/button';
  import { Separator } from '$lib/components/ui/separator';
  import { Spinner } from '$lib/components/ui/spinner';

  const { data } = $props();
  const user = $derived(data.user);

  let currentPage = $state(Number(page.url.searchParams.get('page')) || 1);

  const prevPage = () => {
    if (currentPage > 1) {
      currentPage = currentPage - 1;
    }
  };

  const nextPage = () => {
    currentPage = currentPage + 1;
  };
</script>

<div class="flex">
  <ProfilePic {user} class="h-12 w-12" />
  <div class="ml-4 flex flex-col justify-center">
    <!-- name + handle -->
    <div class="flex gap-3 text-2xl">
      {user.name}
      <a
        rel="external noreferrer noopener"
        target="_blank"
        href="https://github.com/{user.handle}"
        class="text-foreground/50 hover:underline"
      >
        @{user.handle}
      </a>
    </div>
  </div>
</div>
<Separator class="mt-4 mb-4" />

<div class="flex">
  <!-- Sidebar -->
  <div class="mt-2 flex flex-1 items-center gap-2">
    <Button onclick={prevPage} disabled={currentPage === 1}>
      <ChevronLeft />
    </Button>
    <span>Page {currentPage}</span>
    {#await data.questsPromise}
      <Button onclick={nextPage} disabled>
        <ChevronRight />
      </Button>
    {:then result}
      <Button onclick={nextPage} disabled={result.isLastPage}>
        <ChevronRight />
      </Button>
    {/await}
  </div>

  <!-- Quests -->
  <div class="flex-3">
    {#await data.questsPromise}
      <Spinner />
    {:then quests}
      {#each quests.quests as quest (quest)}
        <QuestCard {quest} clickable />
      {/each}
    {/await}
  </div>
</div>
