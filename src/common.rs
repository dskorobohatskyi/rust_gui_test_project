pub struct UserInfo {
    pub name: String,
    pub age: u32,
}

#[derive(Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Home,
    Settings,
    About,
}
