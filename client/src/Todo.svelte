<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    // Function to be called when delete button is clicked
    function deleteTodo() {
        // Emit an event to notify parent component (e.g., +page.svelte)
        dispatch('deleteTodo', { id: todo.id });
    }

    export let todo: TodoItem;
</script>

<div class="p-4 border-b border-gray-200 hover:bg-teal-200 transition duration-300 ease-in-out text-center">
    <p class="text-lg"><strong>Task:</strong> {todo.text}</p>
    <p class="text-sm text-gray-600"><strong>Status:</strong> {todo.completed ? 'Completed' : 'Pending'}</p>
    <p class="text-sm text-gray-400"><strong>Created At:</strong> {todo.created_at.toLocaleString()}</p>
    {#if todo.updated_at}
        <p class="text-sm text-gray-400"><strong>Updated At:</strong> {todo.updated_at.toLocaleString()}</p>
    {/if}
    <button 
        class="delete-btn"
        on:click={deleteTodo}
    >X</button>
</div>