#[cfg(feature = "immediate-mode")]
pub struct UserInfo {
    pub name: String,
    pub age: u32,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub enum ApplicationTab {
    #[default]
    Home,
    Settings,
    About,
}

////////////////////////////////////////////////////////

pub const CHANNELS_COUNT: usize = 9;
pub const INVALID_CHANNEL_INDEX: usize = usize::MAX;
pub const BACKUP_CHANNEL_INDEX: usize = 0;

pub const LOW_INTEGER_LIMIT: u32 = 1;
pub const HIGH_INTEGER_LIMIT: u32 = 100;
pub const SUSPICIOUS_LIMIT: u32 = 75;

#[derive(Default)]
pub struct ChannelInfo {
    pub integer_value: u32,
    pub is_suspicious: bool,
}
