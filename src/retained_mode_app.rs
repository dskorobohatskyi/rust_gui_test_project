// #[cfg(feature = "retained-mode")] // This file is compiled only for `retained-mode`

use crate::common::{Tab, UserInfo};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("This is a dummy Retained Mode for now.");
    Ok(())
}

#[derive(Default)]
pub struct RetainedModeApp {
    #[allow(unused)]
    active_tab: Tab,
    
    #[allow(unused)]
    saved_user_info: Option<UserInfo>,
}
