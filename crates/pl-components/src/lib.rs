pub mod components;
pub mod globals;
pub mod panels;
pub mod theme;

// ── Flat re-exports — call sites just use `use crate::branding::*` ────────────
pub use components::pl_badge::PLBadge;
pub use components::pl_button::{PLButton, PLButtonVariant};
pub use components::pl_frame::PLCard;
pub use components::pl_input::PLTextInput;
pub use components::pl_label::PLLabel;
pub use globals::Color;
