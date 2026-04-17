<script lang="ts">
  import SvelteMarkdown from '@humanspeak/svelte-markdown';
  import { Eye, EyeClosed } from '@lucide/svelte';
  import type { Quest } from '$lib/backend/generated-types';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Toggle } from '$lib/components/ui/toggle';

  let { draft = $bindable() }: { draft: Quest } = $props();
  let detailPreview = $state(false);
</script>

<div class="flex gap-4">
  <h1 class="text-2xl font-semibold">Details</h1>
  <Toggle
    class="transition active:scale-95"
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
