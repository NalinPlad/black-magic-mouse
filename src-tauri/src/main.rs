// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(once_cell)]
use std::sync::OnceLock;

#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use rocket::config::{Config};
use rocket::State;
// use rocket::tls::{TlsConfig, CipherSuite};x
// use rocket::tls::TlsConfig;

use mouce::Mouse;
use mouce::MouseActions;
use mouce::common::MouseButton;

extern crate tauri;

use tauri::Window;
use crate::tauri::Manager;

use std::thread;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn get_local_ip() -> String {
//     let local_ip = get_local_ip::get_local_ip().unwrap();
//     local_ip.to_string()
// }



#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct ClientHTML_Static(&'static [u8]);

static CLIENTHTML: &'static [u8] = include_bytes!("./bmm-client/index.html");

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct ClientHTML(String);

static WINDOW: OnceLock<Window> = OnceLock::new();
#[rocket::main]
async fn main() {

    let _ = tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            _ = WINDOW.set(window);

            #[get("/")]
            fn index() ->  ClientHTML {
                ClientHTML(fs::read_to_string("./src/bmm-client/index.html").unwrap())
            }

            //TODO migrate to websockets
            #[get("/data/<v_x>/<v_y>")]
            fn acceleration(v_x: f64, v_y: f64) {
                let mouse_manager = Mouse::new();
                mouse_manager.move_relative(v_x as i32, v_y as i32);
            }

            #[get("/click/<button>")]
            fn mouse_buttons(button: String) {
                let mouse_manager = Mouse::new();
                match button.as_str() {
                    "left" => mouse_manager.click_button(&MouseButton::Left),
                    "right" => mouse_manager.click_button(&MouseButton::Right),
                    _ => mouse_manager.click_button(&MouseButton::Left),
                };
            }

            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                // let mouse_manager = Mouse::new();
                let _rocket = rocket::build()
                    .mount("/", routes![index, acceleration, mouse_buttons])
                    // .manage(mouse_manager)
                    .launch()
                    .await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
