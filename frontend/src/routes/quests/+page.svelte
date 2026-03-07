<script lang="ts">
	let nums = $state([1, 2, 3, 4]);
	let sum = $derived(nums.reduce((acc, num) => acc + num, 0));

	function onButtonClick() {
		nums.push(nums.length + 1);
	}

	import { Button } from '$lib/components/ui/button/index';
</script>

<p>{nums.join(' + ')} = {sum}</p>

<Button onclick={onButtonClick}>Add number</Button>
