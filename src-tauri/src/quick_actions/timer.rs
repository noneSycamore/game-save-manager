use std::{
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};

use tauri::{App, Manager, State};
use tracing::info;

use super::{quick_backup, QuickActionType};

pub type AutoBackupDuration = AtomicU32;

pub fn setup_timer(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!(target:"rgsm::quick_action::timer","Setting up tray timer.");
    let state: State<Arc<AutoBackupDuration>> = app.state();
    let state = state.inner().clone();
    tauri::async_runtime::spawn(async move {
        let mut counter = 0;
        let mut last = 0;
        loop {
            let duration = state.load(std::sync::atomic::Ordering::Relaxed);
            if last != duration {
                // 如果发生改变，重新计数
                counter = 1;
            }

            // 时间间隔必须大于0，因为0表示禁用
            if duration > 0 && (counter >= duration) {
                quick_backup(QuickActionType::Timer).await;
                counter = 0;
            }

            last = duration;
            std::thread::sleep(Duration::from_secs(60));
            counter += 1;
            counter %= u32::MAX; // 防止溢出
        }
    });
    info!(target:"rgsm::quick_action::timer","Tray timer setup complete.");
    Ok(())
}
