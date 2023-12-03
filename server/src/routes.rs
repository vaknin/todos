use actix_web::{get, post, delete, HttpResponse, web::{Data, self}};
// use anyhow::Context;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use tracing::error;
use crate::{models::{Todo, NewTodo}, types::{ServerState, ServerResult, ServerError}};
use diesel::r2d2;

#[get("/")]
pub async fn get_todos(state: Data<ServerState>) -> ServerResult {
    use crate::schema::todos::dsl::*;
    let conn = &mut state.connection_pool.get()?;
    let results: Vec<Todo> = todos.load(conn)?;
    Ok(HttpResponse::Ok().json(results))
}

// #[put("/todos/<todo_id>", format = "json", data = "<todo_update>")]
// pub async fn update_todo(conn: DbConnection, todo_id: i32, todo_update: Json<NewTodo>) -> Option<Json<Todo>> {
//     use crate::schema::todos::dsl::*;
//     let result = conn.run(move |c| {
//         diesel::update(todos.find(todo_id))
//             .set(&*todo_update)
//             .get_result::<Todo>(c)
//             .ok()
//     }).await;
//     result.map(Json)
// }

#[delete("/{todo_id}")]
pub async fn delete_todo(todo_id: web::Path<i32>, state: Data<ServerState>) -> ServerResult {
    use crate::schema::todos::dsl::*;
    let conn = &mut state.connection_pool.get()?;

    // Deleting the todo item with the specified ID
    let count = diesel::delete(todos.filter(id.eq(*todo_id))).execute(conn)?;

    if count == 0 {
        // No rows were deleted, implying the item was not found
        Ok(HttpResponse::NotFound().finish())
    } else {
        // Successfully deleted
        Ok(HttpResponse::Ok().finish())
    }
}

#[post("/new")]
pub async fn create_todo(new_todo: web::Json<NewTodo>, state: Data<ServerState>) -> ServerResult {
    use crate::schema::todos::dsl::*;
    let conn = &mut state.connection_pool.get()?;

    // Create the new Todo item
    let new_todo_item = NewTodo {
        text: new_todo.text.clone()
    };

    // Insert into database (using Diesel ORM in this example)
    let inserted_todo = diesel::insert_into(todos)
        .values(&new_todo_item)
        .get_result::<Todo>(conn)?;

    Ok(HttpResponse::Created().json(inserted_todo))
}