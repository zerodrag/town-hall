<script lang="ts">
  const { data } = $props();
  const user = $derived(data.targetUser);
</script>

<h1>{user.name}'s profile page</h1>
