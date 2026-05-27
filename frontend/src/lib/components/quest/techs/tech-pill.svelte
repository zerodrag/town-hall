<script lang="ts">
  import { cn } from '$lib/utils';
  import type { ClassValue } from 'tailwind-variants';

  let { class: className, tech }: { class?: ClassValue; tech: string } = $props();
</script>

<span
  class={cn(
    className,
    'flex cursor-pointer items-center rounded-xl bg-primary/20 px-2.5 py-0.5 text-sm font-medium text-primary transition select-none active:scale-90'
  )}
>
  {tech}
</span>
