<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types';

  let { draft = $bindable() }: { draft: Quest } = $props();

  let techInput = $state('');
  let trimInput = $derived(techInput.trim());

  function setTech(e: KeyboardEvent) {
    if ((e.key === ' ' || e.key === 'Enter') && trimInput) {
      e.preventDefault();
      // Only input if not duplicate
      if (!draft.techs.includes(trimInput) && draft.techs.length < 10) {
        draft.techs = [...draft.techs, trimInput];
      }
      techInput = '';
    } else if (e.key === 'Backspace' && techInput === '' && draft.techs.length) {
      e.preventDefault();
      draft.techs = draft.techs.slice(0, -1);
    } else if (draft.techs.length === 10 && e.key !== 'Backspace') {
      e.preventDefault();
    } else if (trimInput.length === 15) {
      e.preventDefault();
    }
  }
</script>

<h1 class="text-2xl font-semibold">Techs</h1>

<div
  class="flex min-h-9 w-full min-w-0 flex-wrap gap-1 rounded-xl border border-input bg-input/30 px-3 py-1 text-base transition outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-[3px] aria-invalid:ring-destructive/20 md:text-sm dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40"
>
  {#each draft.techs as tech (tech)}
    <span
      role="button"
      tabindex="0"
      onclick={() => (draft.techs = draft.techs.filter((t) => t !== tech))}
      onkeydown={(e) => e.key === 'Enter' && (draft.techs = draft.techs.filter((t) => t !== tech))}
      class="flex cursor-pointer items-center rounded-xl bg-primary/20 px-2.5 py-0.5 text-sm font-medium text-primary transition select-none active:scale-90"
    >
      {tech}
    </span>
  {/each}
  <input
    class="min-w-24 flex-1 bg-transparent py-0.5 text-sm outline-none placeholder:text-muted-foreground"
    bind:value={techInput}
    onkeydown={setTech}
    placeholder={draft.techs.length < 10 ? 'Enter Techs here.' : ' | 10 Techs maximum.'}
  />
</div>
