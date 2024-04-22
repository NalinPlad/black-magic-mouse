// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(once_cell)]
use std::sync::OnceLock;

#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

extern crate tauri;

use tauri::Window;
use crate::tauri::Manager;

use std::thread;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct ClientHTML(&'static [u8]);

static CLIENTHTML: &'static [u8] = include_bytes!("./bmm-client/index.html");


static WINDOW: OnceLock<Window> = OnceLock::new();
#[rocket::main]
async fn main() {
    // run rocket server as well in .setup

    let _ = tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            _ = WINDOW.set(window);
            
            // get / return rawHtml file in ./bmm-client/index.html
            #[get("/")]
            fn index() ->  ClientHTML {
                ClientHTML(CLIENTHTML)
            }

            //TODO migrate to websockets
            #[get("/data/<data>")]
            fn acceleration(data: String) {
                // parse x;y
                let mut data = data.split(";");
                let x = data.next().unwrap().parse::<f64>().unwrap();
                let y = data.next().unwrap().parse::<f64>().unwrap();

                // print out data
                println!("x: {}, y: {}", x, y);
            }

            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                let _rocket = rocket::build().mount("/", routes![index]).launch().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
