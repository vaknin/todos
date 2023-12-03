<script lang="ts">
	import { onMount } from "svelte";
	import Todo from "../Todo.svelte";
    import {handleDelete} from "../lib/functions";
    const URL = "http://localhost:8080/todos";
    let todos: TodoItem[] = [];
    let loading: boolean = true;

    onMount(async () => {
        const response = fetch('http://localhost:8080/todos')
        .then(async response => {
            todos = await response.json();
        })
        .catch(e => {
            console.log("error fetching: ", e);
        })
        .finally(() => {
            loading = false;
        });
    });
</script>

<h1 class="text-5xl font-bold p-3 text-center mb-5">Todos</h1>

<div class="flex flex-col items-center justify-center rounded-lg overflow-hidden">
    {#each todos as todo}
        <Todo
            on:deleteTodo={handleDelete}
            {todo}
        />
    {/each}
    {#if !loading && todos.length == 0}
        <div class="text-1xl font-bold p-3 text-center mb-5">
            <p>
                No Entries
            </p>
        </div>
    {/if}
</div>