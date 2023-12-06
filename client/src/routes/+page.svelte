<script lang="ts">
	import { onMount } from "svelte";
	import Todo from "../Todo.svelte";
    import {createTodo, getTodos, handleDeleteRequest} from "../lib/functions";
    const URL = "http://localhost:8080";

    let todos: TodoItem[] = [];
    let loading: boolean = true;
    let showForm = false;
    let newTodoText = '';

    // Function to toggle the form visibility
    function toggleForm() { showForm = !showForm }

    onMount(async () => {
        await refresh();
    });

    async function refresh() {
        const result = await getTodos();
        [todos, loading] = result;
    }

    async function handleUpdateEvent(event: CustomEvent) {
        const { id, text, completed } = event.detail;
        let update: UpdateTodoRequest = {
            completed, text
        };

        let r = await fetch(`${URL}/${id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(update)
        });
        await refresh();
    }

    async function handleDeleteEvent(e: CustomEvent) {
        await handleDeleteRequest(e);
        await refresh();
    }

</script>

<!-- Title -->
<h1 class="text-4xl md:text-6xl font-bold text-gray-800 p-6 text-center select-none">Todos</h1>

<!-- Main Container -->
<div class="flex flex-col items-center justify-start min-h-screen bg-gray-50">
    <!-- Create Todo Button -->
    <div class="w-full flex justify-center p-4">
        <button class="bg-teal-500 hover:bg-teal-400 text-white font-bold py-2 px-4 rounded-full text-xl" on:click={toggleForm}>
            +
        </button>
    </div>

    <!-- Todo items -->
    <div class="w-full max-w-6xl px-4 py-2">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each todos as todo}
                <Todo
                    on:deleteTodo={handleDeleteEvent}
                    on:updateTodo={handleUpdateEvent}
                    {todo}
                />
            {/each}
            {#if !loading && todos.length == 0}
                <div class="text-lg font-semibold p-3 text-center col-span-full">
                    <p>No Entries</p>
                </div>
            {/if}
        </div>
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