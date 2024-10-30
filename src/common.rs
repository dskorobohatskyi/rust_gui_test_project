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

#[derive(Default)]
pub struct ChannelInfo {
    pub integer_value: u32,
    pub is_suspicious: bool,
}
