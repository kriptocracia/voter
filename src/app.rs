use std::collections::HashMap;

use crossterm::event::KeyCode;
use nostr_sdk::prelude::Keys;
use tokio::sync::mpsc;

use crate::config::AppConfig;
use crate::nostr::events::{Election, ElectionResults};
use crate::state::AppState;

/// All possible actions flowing through the app event loop.
#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    KeyPress(KeyCode),
    IdentityCreated(String),
    IdentityUnlocked,
}

/// The screen the app is currently showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    PasswordPrompt,
    ElectionList,
    ElectionDetail { election_id: String },
    Vote { election_id: String },
    Results { election_id: String },
    Settings,
}

/// Whether the app should continue running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldQuit {
    Yes,
    No,
}

/// Central application state.
pub struct App {
    pub screen: Screen,
    pub previous_screen: Option<Screen>,
    pub config: AppConfig,
    pub keys: Option<Keys>,
    pub persistent_state: AppState,
    pub elections: HashMap<String, Election>,
    pub results: HashMap<String, ElectionResults>,
    pub action_tx: mpsc::UnboundedSender<Action>,
    pub show_help: bool,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    // UI state for specific screens
    pub election_list_index: usize,
    pub candidate_list_index: usize,
    pub stv_ranking: Vec<u32>,
    pub token_input: String,
    pub password_input: String,
    pub is_loading: bool,
    pub loading_step: Option<String>,
    pub connected: bool,
}

impl App {
    pub fn new(
        config: AppConfig,
        persistent_state: AppState,
        action_tx: mpsc::UnboundedSender<Action>,
    ) -> Self {
        Self {
            screen: Screen::Welcome,
            previous_screen: None,
            config,
            keys: None,
            persistent_state,
            elections: HashMap::new(),
            results: HashMap::new(),
            action_tx,
            show_help: false,
            status_message: None,
            error_message: None,
            election_list_index: 0,
            candidate_list_index: 0,
            stv_ranking: Vec::new(),
            token_input: String::new(),
            password_input: String::new(),
            is_loading: false,
            loading_step: None,
            connected: false,
        }
    }

    /// Process an action and return whether the app should quit.
    pub fn update(&mut self, action: Action) -> ShouldQuit {
        self.error_message = None;

        match action {
            Action::Quit => return ShouldQuit::Yes,
            Action::KeyPress(key) => self.handle_key(key),
            Action::IdentityCreated(pubkey) => {
                self.status_message = Some(format!("Identity created: {}", &pubkey[..16]));
                self.screen = Screen::ElectionList;
            }
            Action::IdentityUnlocked => {
                self.screen = Screen::ElectionList;
            }
        }

        ShouldQuit::No
    }

    fn handle_key(&mut self, key: KeyCode) {
        // Global keys
        match key {
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
                return;
            }
            KeyCode::Char('q') if !self.is_input_mode() => {
                let _ = self.action_tx.send(Action::Quit);
                return;
            }
            _ => {}
        }

        if self.show_help {
            if key == KeyCode::Esc {
                self.show_help = false;
            }
            return;
        }

        match &self.screen {
            Screen::Welcome => self.handle_welcome_key(key),
            Screen::PasswordPrompt => self.handle_password_key(key),
            Screen::ElectionList => self.handle_election_list_key(key),
            Screen::ElectionDetail { .. } => self.handle_election_detail_key(key),
            Screen::Vote { .. } => self.handle_vote_key(key),
            Screen::Results { .. } => self.handle_results_key(key),
            Screen::Settings => self.handle_settings_key(key),
        }
    }

    fn handle_welcome_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1') | KeyCode::Char('g') => {
                let keys = crate::identity::generate_keypair();
                let path = self.config.identity.path.clone();
                match crate::identity::save_identity(&keys, None, &path) {
                    Ok(()) => {
                        let pubkey = crate::identity::export_public_key(&keys);
                        self.keys = Some(keys);
                        let _ = self.action_tx.send(Action::IdentityCreated(pubkey));
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to save identity: {e}"));
                    }
                }
            }
            KeyCode::Char('2') | KeyCode::Char('i') => {
                // Import identity — TODO
            }
            _ => {}
        }
    }

    fn handle_password_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                let path = self.config.identity.path.clone();
                let password = self.password_input.clone();
                match crate::identity::load_identity(Some(&password), &path) {
                    Ok(keys) => {
                        self.keys = Some(keys);
                        self.password_input.clear();
                        let _ = self.action_tx.send(Action::IdentityUnlocked);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Unlock failed: {e}"));
                        self.password_input.clear();
                    }
                }
            }
            KeyCode::Char(c) => {
                self.password_input.push(c);
            }
            KeyCode::Backspace => {
                self.password_input.pop();
            }
            _ => {}
        }
    }

    fn handle_election_list_key(&mut self, key: KeyCode) {
        let election_count = self.elections.len();
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                if election_count > 0 {
                    self.election_list_index =
                        (self.election_list_index + 1).min(election_count - 1);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.election_list_index = self.election_list_index.saturating_sub(1);
            }
            KeyCode::Enter => {
                if let Some(election_id) = self.sorted_election_ids().get(self.election_list_index)
                {
                    let eid = election_id.clone();
                    self.screen = Screen::ElectionDetail { election_id: eid };
                }
            }
            KeyCode::Char('s') => {
                self.previous_screen = Some(self.screen.clone());
                self.screen = Screen::Settings;
            }
            _ => {}
        }
    }

    fn handle_election_detail_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.token_input.clear();
                self.screen = Screen::ElectionList;
            }
            KeyCode::Char('r') if !self.is_input_mode() => {
                if let Screen::ElectionDetail { ref election_id } = self.screen {
                    let eid = election_id.clone();
                    if self.results.contains_key(&eid) {
                        self.screen = Screen::Results { election_id: eid };
                    }
                }
            }
            KeyCode::Char('v') if !self.is_input_mode() => {
                if let Screen::ElectionDetail { ref election_id } = self.screen {
                    let eid = election_id.clone();
                    if self.persistent_state.get_active_token(&eid).is_some() {
                        self.candidate_list_index = 0;
                        self.stv_ranking.clear();
                        self.screen = Screen::Vote { election_id: eid };
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_vote_key(&mut self, key: KeyCode) {
        if let Screen::Vote { ref election_id } = self.screen {
            let election = self.elections.get(election_id);
            let candidate_count = election.map(|e| e.candidates.len()).unwrap_or(0);
            let is_stv = election.map(|e| e.rules_id == "stv").unwrap_or(false);

            match key {
                KeyCode::Esc => {
                    let eid = election_id.clone();
                    self.screen = Screen::ElectionDetail { election_id: eid };
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if candidate_count > 0 {
                        self.candidate_list_index =
                            (self.candidate_list_index + 1).min(candidate_count - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.candidate_list_index = self.candidate_list_index.saturating_sub(1);
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Some(candidate) =
                        election.and_then(|e| e.candidates.get(self.candidate_list_index))
                    {
                        if is_stv {
                            if !self.stv_ranking.contains(&candidate.id) {
                                self.stv_ranking.push(candidate.id);
                            }
                        } else {
                            self.stv_ranking = vec![candidate.id];
                        }
                    }
                }
                KeyCode::Char('d') if is_stv => {
                    if let Some(candidate) =
                        election.and_then(|e| e.candidates.get(self.candidate_list_index))
                    {
                        self.stv_ranking.retain(|&id| id != candidate.id);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_results_key(&mut self, key: KeyCode) {
        if let Screen::Results { ref election_id } = self.screen
            && key == KeyCode::Esc
        {
            let eid = election_id.clone();
            self.screen = Screen::ElectionDetail { election_id: eid };
        }
    }

    fn handle_settings_key(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            self.go_back();
        }
    }

    fn go_back(&mut self) {
        if let Some(prev) = self.previous_screen.take() {
            self.screen = prev;
        } else {
            self.screen = Screen::ElectionList;
        }
    }

    fn is_input_mode(&self) -> bool {
        matches!(self.screen, Screen::PasswordPrompt)
    }

    /// Returns election IDs sorted by name.
    pub fn sorted_election_ids(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.elections.keys().cloned().collect();
        ids.sort_by(|a, b| {
            let name_a = self.elections.get(a).map(|e| e.name.as_str()).unwrap_or("");
            let name_b = self.elections.get(b).map(|e| e.name.as_str()).unwrap_or("");
            name_a.cmp(name_b)
        });
        ids
    }
}
