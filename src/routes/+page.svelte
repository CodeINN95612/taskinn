<script lang="ts">
	import ProcessesDataTable from '$lib/components/ProcessesDataTable.svelte';
	import type { ProcessInfo } from '$lib/types';
	import { invoke } from '@tauri-apps/api/core';
	import { X } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import type { ChangeEventHandler } from 'svelte/elements';

	let loading = $state(true);
	let get_error = $state('');
	let processes: ProcessInfo[] = $state([]);

	let filter_query = $state('');
	let filtered_processes = $derived.by(() => {
		if (filter_query === '') {
			return processes;
		}
		return processes.filter(
			(process) =>
				process.name.toLowerCase().includes(filter_query) ||
				process.pid.toString().includes(filter_query)
		);
	});

	let selected: ProcessInfo[] = $state([]);

	async function get_processes() {
		try {
			let result = await invoke<ProcessInfo[]>('get_processes');
			processes = result.sort((a, b) => a.memory_bytes - b.memory_bytes);
		} catch (error) {
			if (error instanceof Error) get_error = error.message;
			else get_error = 'An unknown error occurred';
		}
	}

	onMount(() => {
		loading = true;
		get_processes().finally(() => {
			loading = false;
		});

		const interval = setInterval(get_processes, 1000);

		return () => {
			clearInterval(interval);
		};
	});

	const filterProcesses: ChangeEventHandler<HTMLInputElement> = (e) => {
		const target = e.target == null ? null : (e.target as HTMLInputElement);
		const query = target?.value?.toLowerCase() ?? '';
		filter_query = query;
	};
</script>

<header>
	<div class="bg-sidebar-background flex items-center justify-between p-4 text-sidebar-foreground">
		<h1 class="text-xl font-bold">Task Manager</h1>
		<div class="flex items-center space-x-10">
			<input
				type="text"
				placeholder="Search by name or PID"
				class="rounded-md border bg-input px-4 py-2 text-sidebar-foreground"
				oninput={filterProcesses}
			/>
			<button
				class="flex items-center gap-4 rounded-md bg-destructive px-4 py-2 text-destructive-foreground hover:opacity-75 disabled:opacity-50"
				disabled={selected.length === 0}
			>
				<X />
				Shutdown Tasks
			</button>
		</div>
	</div>
</header>

<main class="container">
	{#if loading}
		<p>Loading...</p>
	{:else if get_error !== ''}
		<p>{get_error}</p>
	{:else}
		<p>Count: {filtered_processes.length}</p>
		<ProcessesDataTable processes={filtered_processes} />
	{/if}
</main>
