<script lang="ts">
    export let lp: number;
    export let task: string;
    export let done: boolean;

    $: task = task.trim();
    $: done = done;

    $: console.log({lp, task, done});
</script>

<div class="row">
    <span>{lp}</span>
    <span>{task}</span>
    <input type="checkbox" bind:checked={done} />
</div>

<style>
    .row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem;
        border-bottom: 1px solid #ccc;
    }

    input[type="checkbox"] {
        width: 1.5rem;
        height: 1.5rem;
    }

    input[type="checkbox"]:checked {
        background-color: #000;
    }
</style>