// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod utils;
pub mod auth;
pub mod cursor_reset;
pub mod tray;
pub mod database;

pub use cursor_reset::{
    reset_machine_id,
    switch_account,
    get_machine_ids,
};
