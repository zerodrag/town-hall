<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import QuestCard from '$lib/components/quest/card/quest-card.svelte';
  import TechInput from '$lib/components/quest/techs/tech-input.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import Input from '$lib/components/ui/input/input.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  let query = $state(page.url.searchParams.get('query') || '');
  let techs = $state<string[]>(page.url.searchParams.getAll('techs'));
  let currentPage = $state(Number(page.url.searchParams.get('page')) || 1);
  let limit = $state(Number(page.url.searchParams.get('limit')) || 20);

  function search() {
    const params: string[][] = [];
    if (query) params.push(['query', query]);
    if (currentPage > 1) params.push(['page', String(currentPage)]);
    if (limit !== 20) params.push(['limit', String(limit)]);
    techs.forEach((tech: string) => params.push(['techs', tech]));
    const searchParams = new URLSearchParams(params);
    goto(`/discover?${searchParams.toString()}`);
  }

  function prevPage() {
    if (currentPage > 1) {
      currentPage = currentPage - 1;
      search();
    }
  }

  function nextPage() {
    if (data.quests.length === limit) {
      currentPage = currentPage + 1;
      search();
    }
  }
</script>

<svelte:head>
  <title>Discover Quests</title>
</svelte:head>

<div>
  <form
    onsubmit={(e) => {
      e.preventDefault();
      search();
    }}
  >
    <Input type="text" placeholder="Search quests..." bind:value={query} />
    <TechInput bind:techs />
    <Button type="submit">Search</Button>
  </form>

  <div>
    {#each data.quests as quest (quest.questId)}
      <QuestCard {quest} />
    {/each}
  </div>

  <div>
    <Button onclick={prevPage} disabled={currentPage === 1}>Previous</Button>
    <span>Page {currentPage}</span>
    <Button onclick={nextPage} disabled={data.quests.length < limit}>Next</Button>
  </div>
</div>
