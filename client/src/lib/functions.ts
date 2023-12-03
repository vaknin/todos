const URL = "http://localhost:8080/todos";

export async function handleDelete(event: CustomEvent) {
    const todoId = event.detail.id;
    try {
        console.log(`attempting to delete: ${URL}/${todoId}`);
        const response = await fetch(`${URL}/${todoId}`, {
            method: 'DELETE'
        });
        if (response.ok) {
            return true;
            // Handle successful deletion
            // e.g., remove the todo from the list, show a notification
        }
    } catch (error) {
        console.error('Error deleting todo:', error);
    }
    return false;
}