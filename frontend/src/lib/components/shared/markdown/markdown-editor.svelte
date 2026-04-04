<script lang="ts">
  import { Textarea } from '$lib/components/ui/textarea';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';
  import DOMPurify from 'isomorphic-dompurify';
  import { marked } from 'marked';

  let markdown = $state('');
  let enablePreview = $state(false);
  let cleanHtml = $state('');

  let onEnableMarkdownPreview = async () => {
    if (!enablePreview) return;
    cleanHtml = DOMPurify.sanitize(await marked.parse(markdown));
  };
</script>

<div class="flex gap-2">
  <Switch bind:checked={enablePreview} onCheckedChange={onEnableMarkdownPreview} id="markdown-preview" />
  <Label for="markdown-preview">Preview</Label>
</div>

{#if enablePreview}
  <div class="prose max-w-none rounded-xl border p-8 prose-invert">
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html cleanHtml}
  </div>
{:else}
  <Textarea class="field-sizing-content min-h-64 font-mono" bind:value={markdown} placeholder="type shit" />
{/if}
