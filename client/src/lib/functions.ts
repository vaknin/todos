const URL = "http://localhost:8080";

export async function getTodos(): Promise<[TodoItem[], boolean]> {
    let todos: TodoItem[] = [];
    let loading: boolean = true;
    await fetch(URL)
    .then(async response => {
        todos = await response.json();
    })
    .catch(e => {
        console.log("error fetching: ", e);
    })
    .finally(() => {
        loading = false;
    });
    return [todos, loading];
}

export async function handleDeleteRequest(event: CustomEvent): Promise<boolean> {
    const todoId = event.detail.id;
    try {
        const response = await fetch(`${URL}/${todoId}`, {
            method: 'DELETE'
        });
        if (response.ok) {
            return true;
        }
    } catch (error) {
        console.error('Error deleting todo:', error);
    }
    return false;
}

export async function createTodo(text: string): Promise<TodoItem | boolean> {
    try {
        const newTodoRequest: NewTodo = { text };
        const response = await fetch(`${URL}/new`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(newTodoRequest)
        });
        if (response.ok) {
            let new_todo: TodoItem = await response.json();
            return new_todo;
        }
    } catch (error) {
        console.error('Error creating todo:', error);
    }

    return false;
}