<script lang="ts">
  import type { Quest } from '$lib/backend/generated-types';
  import TechInput from '../techs/tech-input.svelte';

  let { draft = $bindable() }: { draft: Quest } = $props();
</script>

<h1 class="text-2xl font-semibold">Techs</h1>

<TechInput bind:techs={draft.techs} />