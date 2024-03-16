<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  interface Todo {
    id: number;
    title: string;
    completed: boolean;
  }

  let todos: Todo[] = [];
  let title = "";

  async function addTodo(){
    if(title){
      let todo: Todo = await invoke("generate_todo", { title });
      todos.push(todo);
      todos = todos;
      title = "";
    }
  }

  function removeTodo(){
    todos = todos.filter(todo => !todo.completed);
  }

  $: doneTodos = todos.filter(todo => todo.completed).length;
  $: totalTodos = todos.length;
</script>

<div>
  <form class="row" on:submit|preventDefault={addTodo}>
    <input id="todo-input" placeholder="What you have to do..." bind:value={title} />
    <button type="submit">Add</button>
    <button type="button" on:click={removeTodo}>Remove done</button>
  </form>

  <p id="todos-counter">{doneTodos} / {totalTodos}</p>

  <div class="todo">
    <span>ID</span>
    <p>Title</p>
    <span>Done</span>
  </div>

  {#each todos as todo (todo.id)}
    <div class="todo">
      <span>{todo.id}</span>
      <p>{todo.title}</p>
      <input type="checkbox"
      id="todo-{todo.id}"checked={todo.completed}
      on:click={() => todo.completed = !todo.completed}/>
    </div>
  {:else}
    <p>No todos</p>
  {/each}
</div>

<style>
  #todos-counter {
    margin: 1rem 0;
    text-align: right;
  }

  .todo {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    border-bottom: 1px solid #ccc;
  }

  .todo p {
    margin: 0;
  }

  input[type="checkbox"] {
    width: 1.5rem;
    height: 1.5rem;
  }

  input[type="checkbox"]:checked {
    background-color: #000;
  }
</style>