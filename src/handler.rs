use crate::{
    model::{CreateTodoSchema, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
// use chrono::Utc;
use sqlx::PgPool;
// use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, Actix Web, and PostgreSQL";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/todos")]
pub async fn todos_list_handler(
    db: web::Data<PgPool>,
    opts: web::Query<QueryOptions>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(db.get_ref())
    .await
    .unwrap();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    HttpResponse::Ok().json(json_response)
}

#[post("/todos")]
async fn create_todo_handler(
    body: web::Json<CreateTodoSchema>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, content) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.content.to_string()
    )
    .fetch_one(db.get_ref())
    .await
    .unwrap();

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    HttpResponse::Ok().json(json_response)
}

#[get("/todos/{id}")]
async fn get_todo_handler(
    path: web::Path<i32>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_optional(db.get_ref())
        .await
        .unwrap();

    match todo {
        Some(todo) => {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo },
            };
            HttpResponse::Ok().json(json_response)
        }
        None => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with ID: {} not found", id),
            };
            HttpResponse::NotFound().json(error_response)
        }
    }
}

#[patch("/todos/{id}")]
async fn edit_todo_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateTodoSchema>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_optional(db.get_ref())
        .await
        .unwrap();

    if let Some(todo) = todo {
        let updated_todo = sqlx::query_as!(
            Todo,
            "UPDATE todos SET title = $1, content = $2, completed = $3 WHERE id = $4 RETURNING *",
            body.title.as_ref().unwrap_or(&todo.title),
            body.content.as_ref().unwrap_or(&todo.content),
            body.completed.unwrap_or(todo.completed),
            id
        )
        .fetch_one(db.get_ref())
        .await
        .unwrap();

        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: updated_todo },
        };
        HttpResponse::Ok().json(json_response)
    } else {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        HttpResponse::NotFound().json(error_response)
    }
}

#[delete("/todos/{id}")]
async fn delete_todo_handler(
    path: web::Path<i32>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(db.get_ref())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(todos_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler);

    conf.service(scope);
}