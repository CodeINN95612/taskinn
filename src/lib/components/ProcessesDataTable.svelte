<script lang="ts">
	import type { ProcessInfo } from '$lib/types';

	type Props = {
		processes: ProcessInfo[];
	};
	let { processes }: Props = $props();
</script>

<table class="min-w-full divide-y divide-border">
	<thead class="bg-muted">
		<tr>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				<input type="checkbox" />
			</th>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				Process Name
			</th>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				PID
			</th>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				CPU Usage
			</th>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				Memory Usage
			</th>
			<th
				scope="col"
				class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground"
			>
				Disk Usage
			</th>
		</tr>
	</thead>
	<tbody class="divide-y divide-border bg-card">
		{#each processes as process}
			<tr>
				<td class="whitespace-nowrap px-6 py-4">
					<input type="checkbox" />
				</td>
				<td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-foreground">
					{process.name}
				</td>
				<td class="whitespace-nowrap px-6 py-4 text-sm text-muted-foreground">
					{process.pid}
				</td>
				<td class="whitespace-nowrap px-6 py-4 text-sm text-muted-foreground">
					{process.cpu_usage.toFixed(2)}%
				</td>
				<td class="whitespace-nowrap px-6 py-4 text-sm text-muted-foreground">
					{(process.memory_bytes / 1_048_576).toFixed(2)} mb
				</td>
				<td class="whitespace-nowrap px-6 py-4 text-sm text-muted-foreground">
					{(process.disk_usage / 1_048_576).toFixed(2)} mb
				</td>
			</tr>
		{/each}
	</tbody>
</table>
