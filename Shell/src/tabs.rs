// tabs.rs — Tab data
 
#[derive(Clone, Debug)]
pub struct Tab {
    pub id:      usize,
    pub url:     String,
    pub title:   String,
    pub loading: bool,
    pub favicon: Option<char>, // emoji placeholder until real favicons
}
 
impl Tab {
    pub fn new(id: usize, url: impl Into<String>) -> Self {
        let url = url.into();
        Self {
            id,
            title:   Self::title_from_url(&url),
            favicon: Self::favicon_from_url(&url),
            url,
            loading: false,
        }
    }
 
    pub fn title_from_url(url: &str) -> String {
        url.trim_start_matches("https://")
           .trim_start_matches("http://")
           .trim_start_matches("www.")
           .split('/')
           .next()
           .unwrap_or(url)
           .to_string()
    }
 
    fn favicon_from_url(url: &str) -> Option<char> {
        if url.contains("github")    { return Some('🐙'); }
        if url.contains("youtube")   { return Some('▶'); }
        if url.contains("google")    { return Some('🔍'); }
        if url.contains("twitter") ||
           url.contains("x.com")     { return Some('𝕏'); }
        if url.contains("figma")     { return Some('🎨'); }
        if url.contains("discord")   { return Some('💬'); }
        if url.contains("reddit")    { return Some('🤖'); }
        Some('🌐')
    }
}
 
pub struct TabManager {
    pub tabs:       Vec<Tab>,
    pub active:     usize,
    next_id:        usize,
}
 
impl TabManager {
    pub fn new() -> Self {
        let mut mgr = Self { tabs: vec![], active: 0, next_id: 1 };
        // Start with a few demo tabs
        mgr.push("https://google.com");
        mgr.push("https://github.com");
        mgr.push("https://youtube.com");
        mgr.push("https://figma.com");
        mgr.push("https://discord.com");
        mgr.active = 0;
        mgr
    }
 
    pub fn push(&mut self, url: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab::new(id, url));
        id
    }
 
    pub fn new_tab(&mut self) {
        let id = self.push("nova://newtab");
        self.active = self.tabs.len() - 1;
    }
 
    pub fn close(&mut self, idx: usize) {
        if self.tabs.len() <= 1 { return; }
        self.tabs.remove(idx);
        self.active = self.active.min(self.tabs.len() - 1);
    }
 
    pub fn scroll(&mut self, delta: f32) {
        let n = self.tabs.len() as isize;
        let next = (self.active as isize + delta.signum() as isize).rem_euclid(n) as usize;
        self.active = next;
    }
 
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active)
    }
 
    /// Opacity of tab at index based on distance from active
    pub fn tab_opacity(&self, idx: usize) -> f32 {
        let dist = (idx as isize - self.active as isize).unsigned_abs();
        match dist {
            0 => 1.00,
            1 => 0.50,
            2 => 0.25,
            3 => 0.12,
            _ => 0.06,
        }
    }
 
    /// Resolve typed input to a URL
    pub fn resolve(input: &str) -> String {
        let s = input.trim();
        if s.starts_with("http://") || s.starts_with("https://") {
            return s.to_string();
        }
        if !s.contains(' ') && s.contains('.') {
            return format!("https://{}", s);
        }
        format!("https://www.google.com/search?q={}", s.replace(' ', "+"))
    }
}
 