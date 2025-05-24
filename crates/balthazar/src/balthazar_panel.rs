use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    App, AppContext, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Render, Task, WeakEntity, Window, actions,
};
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use ui::{Icon, IconName, Label, Tab, h_flex, prelude::*, v_flex};
use util::{ResultExt, TryFutureExt};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::panel_settings::BalthazarPanelSettings;

const BALTHAZAR_PANEL_KEY: &str = "BalthazarPanel";

actions!(notification_panel, [ToggleFocus]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<BalthazarPanel>(window, cx);
        });
    })
    .detach();
}

#[derive(Serialize, Deserialize)]
struct SerializedBalthazarPanel {
    width: Option<Pixels>,
}

pub struct BalthazarPanel {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
}

impl BalthazarPanel {
    pub fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        cx.new(|cx| {
            let this = Self {
                fs,
                focus_handle: cx.focus_handle(),
                width: None,
                pending_serialization: Task::ready(None),
            };
            this
        })
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        BALTHAZAR_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedBalthazarPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let serialized_panel = if let Some(panel) = cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(BALTHAZAR_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedBalthazarPanel>(&panel)?)
            } else {
                None
            };

            workspace.update_in(cx, |workspace, window, cx| {
                let panel = Self::new(workspace, window, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width.map(|w| w.round());
                        cx.notify();
                    });
                }
                panel
            })
        })
    }
}

impl Focusable for BalthazarPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for BalthazarPanel {}

impl Panel for BalthazarPanel {
    fn persistent_name() -> &'static str {
        "BalthazarPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> workspace::dock::DockPosition {
        BalthazarPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file::<BalthazarPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| settings.dock = Some(position),
        );
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| BalthazarPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<ui::IconName> {
        Some(IconName::Function)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Balthazar Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        8
    }
}

impl Render for BalthazarPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child(
            h_flex()
                .justify_between()
                .px_2()
                .py_1()
                // Match the height of the tab bar so they line up.
                .h(Tab::container_height(cx))
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .child(Label::new("Balthazar App"))
                .child(Icon::new(IconName::Function)),
        )
    }
}
