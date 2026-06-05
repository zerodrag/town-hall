<script lang="ts">
  import { resolve } from '$app/paths';
  import icon from '$lib/assets/town-hall.svg';
</script>

<div class="flex items-center gap-1 text-primary">
  <img src={icon} alt="Town Hall logo" class="h-10 w-10" />
  <a class="text-2xl font-bold" href={resolve('/')}>Good First Project</a>
</div>
