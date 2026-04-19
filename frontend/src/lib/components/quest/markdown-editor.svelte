<script lang="ts">
  import { Eye, EyeClosed } from '@lucide/svelte';
  import { Textarea } from '$lib/components/ui/textarea';
  import { Toggle } from '$lib/components/ui/toggle';
  import DOMPurify from 'isomorphic-dompurify';
  import { marked } from 'marked';

  let { markdown = $bindable() }: { markdown: string } = $props();
  let preview = $state(false);
  let html = $state('');

  const escapeHtml = (str: string) => str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

  const render = async () => {
    const escaped = escapeHtml(markdown);
    html = DOMPurify.sanitize(await marked.parse(escaped), {
      ALLOWED_TAGS: [
        'p',
        'br',
        'hr',
        'h1',
        'h2',
        'h3',
        'h4',
        'h5',
        'h6',
        'blockquote',
        'pre',
        'code',
        'strong',
        'em',
        'del',
        'ul',
        'ol',
        'li',
        'table',
        'thead',
        'tbody',
        'tr',
        'th',
        'td',
        'a',
        'img'
      ],
      ALLOWED_ATTR: ['href', 'src', 'alt']
    });
  };
</script>

<Toggle
  class="w-24 transition active:scale-95"
  aria-label="Toggle markdown preview"
  variant="outline"
  size="sm"
  bind:pressed={preview}
  onclick={render}
>
  {#if preview}
    <Eye />
  {:else}
    <EyeClosed />
  {/if}
  Preview
</Toggle>

{#if preview}
  <div class="prose min-h-64 max-w-none rounded-xl border p-10 prose-invert">
    <!-- eslint-disable svelte/no-at-html-tags -->
    {@html html}
  </div>
{:else}
  <Textarea
    class="min-h-64 font-mono wrap-anywhere"
    bind:value={markdown}
    placeholder="Enter description here. Markdown is enabled!"
  />
{/if}
