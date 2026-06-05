<script lang="ts">
  import { Eye, EyeClosed } from '@lucide/svelte';
  import { Toggle } from '$lib/components/ui/toggle';

  let { preview = $bindable() }: { preview: boolean } = $props();
</script>

<Toggle
  class="w-24 cursor-pointer transition active:scale-95"
  aria-label="Toggle markdown preview"
  variant="outline"
  size="sm"
  bind:pressed={preview}
>
  {#if preview}
    <Eye />
  {:else}
    <EyeClosed />
  {/if}
  Preview
</Toggle>
