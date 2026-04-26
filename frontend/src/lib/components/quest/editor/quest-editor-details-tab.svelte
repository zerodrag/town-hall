<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types';
  import MarkdownEditor from '../markdown-editor.svelte';

  let { draft = $bindable() }: { draft: Quest } = $props();
</script>

<div class="flex gap-4">
  <h1 class="text-2xl font-semibold">Details</h1>
</div>

<MarkdownEditor bind:markdown={draft.details} />
