<script lang="ts">
  import { Textarea } from '$lib/components/ui/textarea';
  import MarkdownRenderer from './markdown-renderer.svelte';

  let { markdown = $bindable(), preview }: { markdown: string; preview: boolean } = $props();
</script>

{#if preview}
  <MarkdownRenderer {markdown}></MarkdownRenderer>
{:else}
  <Textarea
    class="min-h-64 font-mono wrap-anywhere"
    bind:value={markdown}
    placeholder="Enter description here. Markdown is enabled!"
  />
{/if}
