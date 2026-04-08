<script lang="ts">
  import type { User } from '$lib/backend/generated-types';
  import NavbarLinks from './navbar-links.svelte';
  import NavbarLogin from './navbar-login.svelte';
  import NavbarLogo from './navbar-logo.svelte';
  import NavbarPfp from './navbar-pfp.svelte';
  import NavbarQuestButton from './navbar-quest-button.svelte';

  let { me }: { me: User | undefined } = $props();
</script>

<nav class="flex items-center py-6">
  <div class="flex-1">
    <NavbarLogo />
  </div>
  <div class="flex-none">
    <NavbarLinks />
  </div>
  <div class="flex flex-1 justify-end gap-4">
    {#if me}
      <NavbarQuestButton />
      <NavbarPfp user={me} />
    {:else}
      <NavbarLogin />
    {/if}
  </div>
</nav>
