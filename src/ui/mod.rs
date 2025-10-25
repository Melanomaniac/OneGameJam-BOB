pub mod build_bob;

pub use build_bob::setup_build_bob_ui;
pub use build_bob::build_bob_ui_system;
pub use build_bob::on_reset_ui;

pub use build_bob::{HeadSlot, BodySlot, LeftArmSlot, RightArmSlot, LeftLegSlot, RightLegSlot, SlotFilled, ResetBuilderUIEvent};