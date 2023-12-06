interface TodoItem {
    id: number;
    text: string;
    completed: boolean;
    created_at: string;
}

interface NewTodo {
    text: string;
}

interface UpdateTodoRequest {
    text?: string;
    completed?: boolean;
}