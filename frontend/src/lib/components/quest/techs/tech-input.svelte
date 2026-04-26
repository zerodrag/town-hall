<script lang="ts">
  import TechPill from './tech-pill.svelte';

  let { techs = $bindable() }: { techs: string[] } = $props();

  let techInput = $state('');
  let trimInput = $derived(techInput.trim());

  function setTech(e: KeyboardEvent) {
    if ((e.key === ' ' || e.key === 'Enter' || e.key === ',') && trimInput) {
      e.preventDefault();
      if (!techs.includes(trimInput) && techs.length < 10) {
        techs.push(trimInput);
      }
      techInput = '';
    } else if (e.key === 'Backspace' && techInput === '' && techs.length) {
      e.preventDefault();
      techs.pop();
    } else if ((techs.length === 10 || trimInput.length === 15) && e.key !== 'Backspace') {
      e.preventDefault();
    }
  }
</script>

<div
  class="flex min-h-9 w-full min-w-0 flex-wrap gap-1 rounded-xl border border-input bg-input/30 px-3 py-1 text-base transition outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-[3px] aria-invalid:ring-destructive/20 md:text-sm dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40"
>
  {#each techs as tech (tech)}
    <TechPill {tech} enableLink={false} />
  {/each}
  <input
    class="min-w-24 flex-1 bg-transparent py-0.5 text-sm outline-none placeholder:text-muted-foreground"
    bind:value={techInput}
    onkeydown={setTech}
    placeholder={techs.length < 10 ? 'Enter Techs here.' : ' | 10 Techs maximum.'}
  />
</div>
