<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import Todo from "./Todo.svelte";

  let todos: {lp: number, task: string, done: boolean}[] = [
    { lp: 1, task: "Learn Tauri", done: false },
    { lp: 2, task: "Learn Svelte", done: false },
    { lp: 3, task: "Learn Rust", done: false },
  ];

  let task = "";

  function addTodo(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let todo = { lp: todos.length + 1, task, done: false };
    todos.push(todo);
    todos = todos;
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={addTodo}>
    <input id="greet-input" placeholder="What you have to do..." bind:value={task} />
    <button type="submit">Add</button>
    <button type="button" on:click={() => console.log(todos)}>?</button>
  </form>

  {#each todos as todo (todo.lp)}
    <Todo {...todo} />
  {/each}
</div>