<script lang="ts">
  import { Save, Undo2 } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';
  import { fade, fly } from 'svelte/transition';

  let { reset, save }: { reset: () => void; save: () => void } = $props();

  function light_bounce_out(t: number) {
    const c1 = 2.0;
    const c3 = c1 + 1;
    return 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2);
  }
</script>

<div class="flex justify-center">
  <div
    in:fly={{ y: 50, duration: 300, easing: light_bounce_out }}
    out:fade={{ duration: 150 }}
    class="fixed bottom-5 flex w-3xl items-center gap-2 rounded-3xl border bg-card px-5 py-2 shadow-2xl"
  >
    <div>You made unsaved changes.</div>
    <Button size="sm" variant="destructive" class="ml-auto" onclick={reset}>
      <Undo2 />
      <div>Reset</div>
    </Button>
    <Button size="sm" variant="default" onclick={save}>
      <Save />
      <div>Save</div>
    </Button>
  </div>
</div>
