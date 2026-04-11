<script lang="ts">
  import SvelteMarkdown from '@humanspeak/svelte-markdown';
  import { Eye, EyeClosed } from '@lucide/svelte';
  import type { Quest } from '$lib/backend/generated-types';
  import { Input } from '$lib/components/ui/input';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Toggle } from '$lib/components/ui/toggle';

  let { draft = $bindable() }: { draft: Quest } = $props();

  let detailPreview = $state(false);
</script>

<h1 class="text-2xl font-bold">Title</h1>
<Input placeholder="Enter title here." bind:value={draft.title} />

<h1 class="text-2xl font-bold">Summary</h1>
<Input placeholder="Enter summary here." bind:value={draft.summary} />

<div class="flex gap-4">
  <h1 class="text-2xl font-bold">Details</h1>
  <Toggle
    class="transition-all active:scale-95"
    aria-label="Toggle markdown preview"
    variant="outline"
    size="sm"
    bind:pressed={detailPreview}
  >
    {#if detailPreview}
      <Eye />
    {:else}
      <EyeClosed />
    {/if}
    Preview
  </Toggle>
</div>

{#if detailPreview}
  <div class="prose min-h-64 max-w-none rounded-xl border p-10 prose-invert">
    <SvelteMarkdown source={draft.details} />
  </div>
{:else}
  <Textarea
    class="min-h-64 font-mono wrap-anywhere"
    bind:value={draft.details}
    placeholder="Enter description here. Markdown is enabled!"
  />
{/if}
