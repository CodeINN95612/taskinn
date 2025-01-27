<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	type ProcessInfo = {
		pid: number;
		name: string;
		cpu_usage: number;
		memory_bytes: number;
	};

	async function get_processes() {
		return await invoke<ProcessInfo[]>('get_processes');
	}
</script>

<main class="container">
	{#await get_processes()}
		<p>...Loading</p>
	{:then processes}
		<h1>Processes</h1>
		<div class="grid gap-4">
			{#each processes as process}
				<div>
					<p>{process.name}</p>
					<p>{process.pid}</p>
					<p>{process.cpu_usage}</p>
					<p>{process.memory_bytes}</p>
				</div>
			{/each}
		</div>
	{:catch error}
		<p>{error.message}</p>
	{/await}
</main>
