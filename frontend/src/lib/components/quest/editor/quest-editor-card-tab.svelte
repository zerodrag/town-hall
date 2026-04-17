<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types';
  import { Input } from '$lib/components/ui/input';
  import { Textarea } from '$lib/components/ui/textarea';

  let { draft = $bindable() }: { draft: Quest } = $props();
</script>

<h1 class="text-2xl font-semibold">Title</h1>
<Input placeholder="Enter title here." bind:value={draft.title} />

<h1 class="text-2xl font-semibold">Summary</h1>
<Textarea rows={2} class="max-h-2 wrap-anywhere" placeholder="Enter summary here." bind:value={draft.summary} />
