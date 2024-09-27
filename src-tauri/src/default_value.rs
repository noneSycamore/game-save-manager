use crate::{
    cloud_sync::{Backend, CloudSettings},
    config::QuickActionsSettings,
};

pub fn default_false() -> bool {
    false
}
pub fn default_true() -> bool {
    true
}
pub fn default_zero() -> u64 {
    0
}
pub fn default_root_path() -> String {
    "/game-save-manager".to_string()
}
pub fn default_home_page() -> String {
    "/home".to_string()
}
pub fn default_backend() -> Backend {
    Backend::Disabled
}
pub fn default_cloud_settings() -> CloudSettings {
    CloudSettings::default()
}
pub fn default_locale() -> String {
    "zh_SIMPLIFIED".to_owned()
}
pub fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}
pub fn default_none<T>() -> Option<T> {
    None
}
pub fn default_quick_action_settings() -> QuickActionsSettings {
    QuickActionsSettings::default()
}
