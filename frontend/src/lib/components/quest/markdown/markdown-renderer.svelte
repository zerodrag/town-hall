<script lang="ts">
  import { cn } from '$lib/utils';
  import DOMPurify from 'isomorphic-dompurify';
  import { marked } from 'marked';

  let { markdown, class: className = '' }: { markdown: string; class?: string } = $props();

  const escapeHtml = (str: string) => str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  const render = async (markdown: string) => {
    const escaped = escapeHtml(markdown);
    return DOMPurify.sanitize(await marked.parse(escaped), {
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

  let htmlPromise = $derived(render(markdown));
</script>

<div class={cn(className, 'prose prose-invert')}>
  {#await htmlPromise then html}
    <!-- eslint-disable svelte/no-at-html-tags -->
    {@html html}
  {/await}
</div>
