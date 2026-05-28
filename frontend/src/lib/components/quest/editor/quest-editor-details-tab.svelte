<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types';
  import { MarkdownEditor, MarkdownToggle } from '$lib/components/quest/markdown';

  let { draft = $bindable() }: { draft: Quest } = $props();

  let preview = $state(false);
</script>

<div class="flex gap-4">
  <h1 class="text-2xl font-semibold">Details</h1>
  <MarkdownToggle bind:preview />
</div>

<MarkdownEditor markdown={draft.details} {preview} />
