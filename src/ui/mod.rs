use crate::layout_config::LayoutConfig;

pub mod overlay_infos;
pub mod sidebar;

pub enum OverlaySignals {
    ChangeActiveState(bool, LayoutConfig),
    Save(LayoutConfig),
    Remove(LayoutConfig),
}

