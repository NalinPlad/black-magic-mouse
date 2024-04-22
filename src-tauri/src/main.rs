// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![feature(once_cell)]
use std::sync::OnceLock;

#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use rocket::config::{Config};
// use rocket::tls::{TlsConfig, CipherSuite};x
// use rocket::tls::TlsConfig;

use mouse_rs::{Mouse};

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


// static mut v_x: f64 = 0.0;
// static mut v_y: f64 = 0.0;

// static mut x: f64 = 0.0;
// static mut y: f64 = 0.0;


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
                // read index.html to a string
                use std::env;

                // We assume that we are in a valid directory.
                // let path = env::current_dir().unwrap();
                // println!("The current directory is {}", path.display());

                ClientHTML(fs::read_to_string("./src/bmm-client/index.html").unwrap())
            }

            //TODO migrate to websockets
            #[get("/data/<v_x>/<v_y>")]
            fn acceleration(v_x: f64, v_y: f64) {


                // print out data
                // println!("x: {}, y: {}", a_x, a_y);

                // unsafe {
                //     println!("vx: {}, vy: {}", v_x, v_y);
                //     v_x += x;
                //     v_y += y;
                // }

                let mut mouse = Mouse::new();
                let pos = mouse.get_position().unwrap();

                // floor accell data
                // let move_x = x.floor() as i32 + pos.x;
                // let move_y = y.floor() as i32 + pos.y;

                // move mouse by amount
                unsafe{
                    mouse.move_to(pos.x + v_x as i32, pos.y + v_y as i32);
                }


            }

            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                // config with tls certs

                // let tls = TlsConfig::from_paths(
                //     "./keys/cert.pem",
                //     "./keys/key.pem"


                let _rocket = rocket::build()
                    .mount("/", routes![index, acceleration])
                    .launch()
                    .await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
