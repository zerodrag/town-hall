<script lang="ts">
  import { SiGithub } from '@icons-pack/svelte-simple-icons';
  import { LogIn } from '@lucide/svelte';
  import { BACKEND_URL } from '$lib/backend/common';
  import { Button } from '$lib/components/ui/button/';
</script>

<Button href="{BACKEND_URL}/auth/github">
  <LogIn />Sign in with <SiGithub /> GitHub
</Button>
