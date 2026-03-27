<script lang="ts">
	const { data } = $props();
	const quest = $derived(data.quest);
</script>

<h1>{quest.title}</h1>
<p>{quest.description}</p>
