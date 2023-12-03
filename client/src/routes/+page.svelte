<script lang="ts">
	import { onMount } from "svelte";
	import Todo from "../Todo.svelte";
    import {createTodo, handleDelete} from "../lib/functions";
    const URL = "http://localhost:8080";

    let todos: TodoItem[] = [];
    let loading: boolean = true;
    let showForm = false;
    let newTodoText = '';

    // Function to toggle the form visibility
    function toggleForm() { showForm = !showForm }

    onMount(async () => {
        const response = fetch(URL)
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

<!-- Title -->
<h1 class="text-6xl font-bold p-6 text-center select-none">Todos</h1>

<!-- Main Container -->
<div class="flex flex-col items-center justify-start min-h-screen">
    <!-- Create Todo Button -->
    <div class="w-full flex justify-center p-4">
        <button class="bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded" on:click={toggleForm}>
            +
        </button>
    </div>

    <!-- Todo items -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 2xl:grid-cols-6 gap-4 p-4 max-h-screen overflow-hidden">
        {#each todos as todo}
            <Todo
                on:deleteTodo={async e => {
                    let deleted = await handleDelete(e);
                    if (deleted) {
                        todos = todos.filter(t => e.detail.id != t.id);
                    }
                }}
                {todo}
            />
        {/each}
        {#if !loading && todos.length == 0}
            <div class="text-xl font-bold p-3 text-center mb-5 col-span-full">
                <p>No Entries</p>
            </div>
        {/if}
    </div>

</div>

<!-- Create Todo Form -->
{#if showForm}
<div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex justify-center items-center">
    <div class="bg-white p-5 rounded-lg shadow-lg">
        <input class="border p-2" type="text" bind:value={newTodoText} placeholder="Enter Todo">

        <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded m-2" on:click={async () => {
            let created = await createTodo(newTodoText);
            if (typeof(created) !== "boolean") {
                todos = [...todos, created];
            }
            newTodoText = '';
            showForm = false;
        }}>
            Confirm
        </button>

        <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded m-2" on:click={toggleForm}>
            Cancel
        </button>
    </div>
</div>
{/if}