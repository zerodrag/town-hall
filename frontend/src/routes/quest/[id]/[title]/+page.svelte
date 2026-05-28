<script lang="ts">
  import { page } from '$app/state';
  import MarkdownRenderer from '$lib/components/quest/markdown/markdown-renderer.svelte';
  import { Button } from '$lib/components/ui/button';

  const { data } = $props();
  let quest = $derived(data.quest);
  let me_owns_quest = $derived(data.me?.userId === quest.poster.userId);
</script>

{#if me_owns_quest}
  <Button href="{page.url.pathname}/edit">Edit quest</Button>
{/if}

<h1 class="text-4xl font-extrabold">{quest.title}</h1>
<p class="text-xl">{quest.summary}</p>
<MarkdownRenderer markdown={quest.details} />
