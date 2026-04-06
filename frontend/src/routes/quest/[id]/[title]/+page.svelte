<script lang="ts">
  import { page } from '$app/state';
  import { Button } from '$lib/components/ui/button';

  const { data } = $props();
  let quest = $derived(data.quest);
  let me_owns_quest = $derived(data.me?.userId === quest.posterId);
</script>

<h1 class="text-4xl font-extrabold">{quest.title}</h1>
<p class="text-xl">{quest.description}</p>

{#if me_owns_quest}
  <Button href="{page.url.pathname}/edit">Edit quest</Button>
{/if}
