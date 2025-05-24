use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::Pixels;
use workspace::dock::DockPosition;

#[derive(Deserialize, Debug)]
pub struct BalthazarPanelSettings {
    pub dock: workspace::dock::DockPosition,
    pub default_width: Pixels,
}

impl Settings for BalthazarPanelSettings {
    const KEY: Option<&'static str> = Some("balthazar_panel");

    type FileContent = PanelSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct PanelSettingsContent {
    pub dock: Option<DockPosition>,
    pub default_width: Option<f32>,
}
