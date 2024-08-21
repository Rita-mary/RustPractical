use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    id: Uuid,
    description: String,
    completed: bool,
}

#[derive(Deserialize)]
struct NewTask {
    description: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTask {
    description: Option<String>,
    completed: Option<bool>,
}

async fn create_task(state: web::Data<AppState>, new_task: web::Json<NewTask>) -> impl Responder {
    let mut tasks = state.tasks.lock().unwrap();

    let task = Task {
        id: Uuid::new_v4(),
        description: new_task.description.clone(),
        completed: new_task.completed,
    };

    tasks.push(task.clone());

    HttpResponse::Ok().json(task)
}

async fn get_tasks(state: web::Data<AppState>) -> impl Responder {
    let tasks = state.tasks.lock().unwrap();
    HttpResponse::Ok().json(tasks.clone())
}

async fn update_task(
    state: web::Data<AppState>,
    task_id: web::Path<Uuid>,
    updated_task: web::Json<UpdateTask>,
) -> impl Responder {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == *task_id) {
        if let Some(description) = &updated_task.description {
            task.description = description.clone();
        }
        if let Some(completed) = updated_task.completed {
            task.completed = completed;
        }
        HttpResponse::Ok().json(task.clone())
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

async fn delete_task(state: web::Data<AppState>, task_id: web::Path<Uuid>) -> impl Responder {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(pos) = tasks.iter().position(|t| t.id == *task_id) {
        tasks.remove(pos);
        HttpResponse::Ok().body("Task deleted")
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
