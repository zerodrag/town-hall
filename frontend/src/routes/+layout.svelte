<script lang="ts">
  import './layout.css';
  import favicon from '$lib/assets/town-hall.svg';
  import { Footer } from '$lib/components/footer';
  import { Navbar } from '$lib/components/navbar';

  let { data, children } = $props();

  const me = $derived(data.me);
</script>

<svelte:head>
  <title>GFP</title>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="flex min-h-screen flex-col">
  <div class="mx-auto w-full max-w-7xl flex-1 px-6 pb-10">
    <Navbar {me} />
    {@render children()}
  </div>
  <Footer />
</div>
