<script lang="ts">

    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    export let todo: TodoItem;
    let editText: string = todo.text;

    // Function to dispatch the update event
    function updateTodo() { dispatch('updateTodo', { id: todo.id, text: editText, completed: todo.completed }) }

    // Function to be called when delete button is clicked
    function deleteTodo() { dispatch('deleteTodo', { id: todo.id }) }

</script>

<div class="flex flex-col justify-between p-4 border border-gray-200 hover:bg-gray-100 transition duration-300 ease-in-out rounded-md">
    <div>
        <p class="text-sm font-semibold text-gray-700">{todo.completed ? 'Completed' : 'Pending'}</p>
        <p class="text-xs text-gray-400">Created: {todo.created_at.toLocaleString()}</p>
        {#if todo.updated_at}
            <p class="text-xs text-gray-400">Updated: {todo.updated_at.toLocaleString()}</p>
        {/if}
    </div>

    <div class="flex items-center space-x-2 mt-4">
        <input type="checkbox" class="form-checkbox h-5 w-5 text-teal-600" bind:checked={todo.completed} on:change={updateTodo} />
        <input type="text" class="form-input px-2 py-1 border-gray-300 rounded-md text-sm flex-1" bind:value={editText} on:blur={updateTodo} />
        <button 
            class="delete-btn px-2 py-1 bg-red-500 hover:bg-red-600 text-white font-medium rounded-md"
            on:click={deleteTodo}
        >X</button>
    </div>
</div>
