pub mod balthazar_panel;
mod panel_settings;
use gpui::App;
use panel_settings::BalthazarPanelSettings;
use settings::Settings;

pub use balthazar_panel::BalthazarPanel;

pub fn init(cx: &mut App) {
    BalthazarPanelSettings::register(cx);

    balthazar_panel::init(cx);
}
