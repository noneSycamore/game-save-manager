#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate rust_i18n;
use rust_i18n::t;
i18n!("../locales", fallback = ["en_US", "zh_SIMPLIFIED"]);

use config::get_config;

use std::sync::{Arc, Mutex};
use tauri::api::notification::Notification;
use tracing::{error, info};
use tracing_subscriber::{filter::LevelFilter, Layer};

use crate::config::config_check;

mod archive;
mod backup;
mod cloud;
mod config;
mod default_value;
mod errors;
mod ipc_handler;
mod tray;
mod traits;

fn main() {
    init_log();
    info!("{}", t!("home.hello_world"));
    let app = tauri::Builder::default()
        .manage(Arc::new(Mutex::new(tray::QuickBackupState::default())))
        .invoke_handler(tauri::generate_handler![
            ipc_handler::open_url,
            ipc_handler::choose_save_file,
            ipc_handler::choose_save_dir,
            ipc_handler::get_local_config,
            ipc_handler::add_game,
            ipc_handler::apply_backup,
            ipc_handler::delete_backup,
            ipc_handler::delete_game,
            ipc_handler::get_backup_list_info,
            ipc_handler::set_config,
            ipc_handler::reset_settings,
            ipc_handler::backup_save,
            ipc_handler::open_backup_folder,
            ipc_handler::check_cloud_backend,
            ipc_handler::cloud_upload_all,
            ipc_handler::cloud_download_all,
            ipc_handler::set_backup_describe,
            ipc_handler::backup_all,
            ipc_handler::apply_all,
            ipc_handler::set_quick_backup_game,
            ipc_handler::get_locale_message
        ]);

    // 只允许运行一个实例
    let app = app.plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}));
    if let Err(e) = config_check() {
        error!("Check on config file filed: {}", e);
        panic!("Check on config file filed.");
    }

    // 处理退出到托盘
    if let Ok(config) = get_config() {
        if config.settings.exit_to_tray {
            app.system_tray(tray::get_tray())
                .on_system_tray_event(tray::tray_event_handler)
                .setup(tray::setup_timer)
                .build(tauri::generate_context!())
                .expect("Cannot build tauri app")
                .run(|_app_handle, event| {
                    if let tauri::RunEvent::ExitRequested { api, .. } = event {
                        api.prevent_exit();
                    }
                });
            return;
        }
    }
    // 不需要退出到托盘
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");

    // 需要初始化Notification，否则第一次提示不会显示
    Notification::new("Init Info")
        .title("Init")
        .body("Initiating notification module")
        .show()
        .expect("Cannot show notification");
}

fn init_log(){
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let file_appender = tracing_appender::rolling::daily("./log", "RGSM");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_filter(LevelFilter::INFO),
        )
        .init();
}
