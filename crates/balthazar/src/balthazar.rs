pub mod balthazar_panel;
mod panel_settings;
use gpui::App;
use panel_settings::BalthazarPanelSettings;
use settings::Settings;
use std::sync::Arc;
use workspace::AppState;

pub use balthazar_panel::BalthazarPanel;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    BalthazarPanelSettings::register(cx);

    balthazar_panel::init(cx);
}
