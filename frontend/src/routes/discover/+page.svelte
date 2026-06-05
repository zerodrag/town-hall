<script lang="ts">
  import { ChevronLeft, ChevronRight, Search, TextSearch } from '@lucide/svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import QuestCard from '$lib/components/quest/card/quest-card.svelte';
  import TechInput from '$lib/components/quest/techs/tech-input.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import Input from '$lib/components/ui/input/input.svelte';
  import { Spinner } from '$lib/components/ui/spinner';

  let { data } = $props();

  let query = $state(page.url.searchParams.get('query') || '');
  let techs = $state(page.url.searchParams.getAll('techs'));
  let currentPage = $state(Number(page.url.searchParams.get('page')) || 1);
  let limitString = $state(page.url.searchParams.get('limit') || '20');
  let limit = $derived(Number(limitString));

  const search = async () => {
    const params: string[][] = [];
    if (query) params.push(['query', query]);
    if (currentPage > 1) params.push(['page', String(currentPage)]);
    if (limit !== 20) params.push(['limit', String(limit)]);
    techs.forEach((tech: string) => params.push(['techs', tech]));
    const searchParams = new URLSearchParams(params);
    await goto(`/discover?${searchParams.toString()}`, {
      replaceState: true,
      keepFocus: true
    });
  };

  const addTech = (tech: string) => {
    if (!techs.includes(tech)) {
      techs.push(tech);
      search();
    }
  };

  const prevPage = () => {
    if (currentPage > 1) {
      currentPage = currentPage - 1;
      search();
    }
  };

  const nextPage = () => {
    currentPage = currentPage + 1;
    search();
  };

  const searchFormId = 'discover-search-form';
</script>

<form
  id={searchFormId}
  onsubmit={(e) => {
    e.preventDefault();
    search();
  }}
></form>

<div class="flex gap-2">
  <div class="flex-1">
    <TechInput bind:techs onInputChange={search} />
  </div>
  <div class="flex flex-3 flex-col gap-2">
    <div class="flex gap-2">
      <Input form={searchFormId} type="text" placeholder="Search quests..." bind:value={query} />
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Button {...props} variant="outline">
              <TextSearch />View
            </Button>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="max-w-4">
          <DropdownMenu.RadioGroup bind:value={limitString} onValueChange={search}>
            <DropdownMenu.RadioItem value="5">5</DropdownMenu.RadioItem>
            <DropdownMenu.RadioItem value="10">10</DropdownMenu.RadioItem>
            <DropdownMenu.RadioItem value="20">20</DropdownMenu.RadioItem>
            <DropdownMenu.RadioItem value="50">50</DropdownMenu.RadioItem>
          </DropdownMenu.RadioGroup>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
      <Button form={searchFormId} type="submit">
        <Search />Search
      </Button>
    </div>

    <div class="flex flex-col gap-3">
      {#await data.questsPromise}
        <Spinner size={100} />
      {:then result}
        {#each result.quests as quest (quest)}
          <QuestCard {quest} {addTech} clickable />
        {/each}
      {/await}
    </div>

    <div class="flex items-center justify-end gap-2">
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
  </div>
</div>
