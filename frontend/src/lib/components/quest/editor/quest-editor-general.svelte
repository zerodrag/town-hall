<script lang="ts">
  import { Textarea } from '$lib/components/ui/textarea';
  import { Input } from '$lib/components/ui/input';
  import SvelteMarkdown from '@humanspeak/svelte-markdown';
  import type { Quest } from '$lib/backend/generated-types';
  import { Toggle } from '$lib/components/ui/toggle';
  import { Eye, EyeClosed } from '@lucide/svelte';

  let { draft = $bindable() }: { draft: Quest } = $props();

  let descriptionPreview = $state(false);
</script>

<h1 class="text-2xl font-bold">Title</h1>
<Input placeholder="Enter title here." bind:value={draft.title} />

<div class="flex gap-4">
  <h1 class="text-2xl font-bold">Description</h1>
  <Toggle
    class="transition-all active:scale-95"
    aria-label="Toggle markdown preview"
    variant="outline"
    size="sm"
    bind:pressed={descriptionPreview}
  >
    {#if descriptionPreview}
      <Eye />
    {:else}
      <EyeClosed />
    {/if}
    Preview
  </Toggle>
</div>

{#if descriptionPreview}
  <div class="prose min-h-64 max-w-none rounded-xl border p-10 prose-invert">
    <SvelteMarkdown source={draft.description} />
  </div>
{:else}
  <Textarea
    class="min-h-64 font-mono wrap-anywhere"
    bind:value={draft.description}
    placeholder="Enter description here. Markdown is enabled!"
  />
{/if}
