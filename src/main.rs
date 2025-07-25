use escpos::driver::*;
use escpos::errors::Result as PrintResult;
use escpos::printer::Printer;
use escpos::utils::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    error::Error,
    net::Ipv4Addr,
};

use actix_web::{
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use utoipa::{
    OpenApi,
};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::todo::TodoStore;

mod todo;

#[derive(Serialize, Deserialize)]
struct JobInfo {
    title: String,
    description: String,
}

impl JobInfo {
    fn dprint(&self) -> PrintResult<()> {
        println!("{}", &self.title);
        println!("{}", &self.description);
        Ok(())
    }

    fn print(&self) -> PrintResult<()> {
        let driver = UsbDriver::open(0x0416, 0x5011, None, None)?;
        Printer::new(driver, Protocol::default(), None)
            .debug_mode(Some(DebugMode::Dec))
            .init()?
            .justify(JustifyMode::CENTER)?
            .size(2, 2)?
            .underline(UnderlineMode::Single)?
            .bold(true)?
            .writeln(&self.title)?
            .feed()?
            .feed()?
            .reset_style_state()
            .justify(JustifyMode::LEFT)?
            .size(1, 1)?
            .underline(UnderlineMode::None)?
            .bold(false)?
            .writeln(&self.description)?
            .feed()?
            .print_cut()?;

        Ok(())
    }
}

fn lsusb() {
    // List of USB devices
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!(
            "Bus: {:03} Device: {:03} VID: {:04x} PID: {:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
        );
    }
}

fn print_from_file(file: &str) ->  Result<(), Box<dyn std::error::Error>>{
 let data = fs::read_to_string("jobs.json")?;

    let j1: Vec<JobInfo> = serde_json::from_str(&data)?;
    for j in j1.iter() {
        j.dprint()?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let store = Data::new(TodoStore::default());
        HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(utoipa_actix_web::scope("/api/todo").configure(todo::configure(store.clone())))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
