<script lang="ts">
  import { Textarea } from '$lib/components/ui/textarea';
  import { cn } from '$lib/utils';
  import MarkdownRenderer from './markdown-renderer.svelte';

  let {
    markdown = $bindable(),
    preview,
    class: className
  }: { markdown: string; preview: boolean; class?: string } = $props();
</script>

{#if preview}
  <MarkdownRenderer {markdown} class={cn('min-h-64 max-w-none rounded-xl border p-10', className)} />
{:else}
  <Textarea
    class={cn('font-mono wrap-anywhere', className)}
    bind:value={markdown}
    placeholder="Enter description here. Markdown is enabled!"
  />
{/if}
