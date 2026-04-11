// input.rs — Raw events → clean Actions
 
#[derive(Debug, Clone)]
pub enum Action {
    // Tab strip
    CtrlHeld,
    CtrlReleased,
    ScrollTabs(f32),
 
    // Island
    OpenIsland,
    CloseIsland,
    IslandType(char),
    IslandBackspace,
    IslandSubmit,
 
    // Tabs
    NewTab,
    CloseTab(usize),
    SwitchTab(usize),
 
    // Settings
    ToggleSettings,
 
    // Window
    Resize(u32, u32),
    Quit,
}