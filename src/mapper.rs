use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub details: String,
    pub state: String,
    pub small_image: Option<String>,
}

impl Status {
    pub fn simple(details: impl Into<String>) -> Self {
        Self {
            details: details.into(),
            state: "Ritual active".to_string(),
            small_image: None,
        }
    }
}

pub struct StaticMapper {
    known_apps: HashMap<&'static str, &'static str>,
}

impl StaticMapper {
    pub fn new() -> Self {
        Self {
            known_apps: HashMap::from([
                ("code", "Writing questionable code"),
                ("vscode", "Writing questionable code"),
                ("firefox", "Surfing the chaos"),
                ("chrome", "Browsing the chaos"),
                ("steam", "Avoiding responsibilities"),
                ("fl_studio", "Cooking distortion"),
                ("reaper", "Mixing something loud"),
                ("spotify", "Listening to epic tunes"),
                ("vlc", "Watching media"),
                ("mpv", "Watching media"),
                ("gimp", "Editing pixels"),
                ("krita", "Painting digital nightmares"),
                ("obs", "Recording chaos"),
                ("discord", "Chatting with mortals"),
                ("telegram", "Messaging in shadows"),
                ("signal", "Encrypted talk"),
                ("zoom", "Meeting the void"),
                ("kitty", "Typing command spells"),
                ("alacritty", "Typing command spells"),
                ("tmux", "Commanding multiple panes"),
                ("htop", "Watching processes die"),
                ("glances", "Monitoring lifeforms"),
                ("nautilus", "Browsing files"),
                ("dolphin", "Browsing files"),
                ("tor_browser", "Surfing safely"),
                ("brave", "Browsing the web"),
                ("vivaldi", "Browsing the web"),
                ("intellij", "Writing questionable code"),
                ("pycharm", "Writing questionable code"),
                ("sublime_text", "Writing questionable code"),
                ("vim", "Editing ancient scrolls"),
                ("neovim", "Editing ancient scrolls"),
                ("minecraft", "Building digital worlds"),
                ("factorio", "Automating everything"),
                ("heroic", "Playing epic games"),
                ("lutris", "Gaming adventures"),
                ("docker", "Spawning containers"),
                ("postman", "Testing HTTP rituals"),
                ("dbeaver", "Exploring databases"),
                ("libreoffice_writer", "Writing documents"),
                ("libreoffice_calc", "Crunching numbers"),
                ("libreoffice_impress", "Presenting visions"),
                ("cheese", "Observing yourself"),
                ("system_settings", "Tuning reality"),
            ]),
        }
    }

    pub fn map(&self, app: &str) -> Option<Status> {
        self.known_apps.get(app).map(|msg| Status {
            details: (*msg).to_string(),
            state: "Focused".to_string(),
            small_image: Some(app.to_string()),
        })
    }
}
