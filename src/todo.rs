use std::sync::Mutex;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use utoipa_actix_web::service_config::ServiceConfig;


#[derive(Default)]
pub(super) struct TodoStore {
    todos: Mutex<Vec<Todo>>,
}
use escpos::driver::*;
use escpos::errors::Result as PrintResult;
use escpos::printer::Printer;
use escpos::utils::*;

const TODO: &str = "todo";

pub(super) fn configure(store: Data<TodoStore>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(create_todo);
    }
}

fn print(job: Todo, debug: bool) -> PrintResult<()> {
    if debug {
        println!("{}", job.title);
        println!("{}", job.description);
    } else {
        
        let driver = UsbDriver::open(0x0416, 0x5011, None, None)?;
        Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .justify(JustifyMode::CENTER)?
        .size(2, 2)?
        .underline(UnderlineMode::Single)?
        .bold(true)?
        .writeln(&job.title)?
        .feed()?
        .feed()?
        .reset_style_state()
        .justify(JustifyMode::LEFT)?
        .size(1, 1)?
        .underline(UnderlineMode::None)?
        .bold(false)?
        .writeln(&job.description)?
        .feed()?
        .print_cut()?;
    }
    Ok(())
}

/// Task to do.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct Todo {
    /// Description of the tasks to do.
    #[schema(example = "Washing Up")]
    title: String,
    #[schema(example = "Wash, dry and put all dishes away.")]
    /// The text contents of the task
    description: String,
}


/// Create new Todo to shared in-memory storage.
///
/// Post a new `Todo` in request body as json to print it. Api will return
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"title": "Do Washing up", "description": "Wash up the dishes"}'
/// ```
#[utoipa::path(
    tag = TODO,
    responses(
        (status = 201, description = "Todo printed successfully", body = Todo),
    )
)]
#[post("")]
async fn create_todo(todo: Json<Todo>, todo_store: Data<TodoStore>) -> impl Responder {
    let mut todos = todo_store.todos.lock().unwrap();
    let todo = &todo.into_inner();
    todos.push(todo.clone());
    match print(todo.clone(), false){
        Ok(_) => HttpResponse::Created().json(todo),
        Err(error) =>  HttpResponse::InternalServerError().body(format!("{error:?}")),
    }
    
}
