use std::fs;

use voter::config::AppConfig;

#[test]
fn default_config_has_relays() {
    let config = AppConfig::default();
    assert!(
        !config.nostr.relays.is_empty(),
        "default relays should not be empty"
    );
}

#[test]
fn missing_file_creates_default() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("voter.toml");

    let config = AppConfig::load(&path).unwrap();
    assert!(path.exists(), "config file should be created");
    assert!(!config.nostr.relays.is_empty());
}

#[test]
fn load_custom_config() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("voter.toml");

    let custom = r#"
[nostr]
relays = ["wss://custom.relay"]

[ui]
theme = "light"
"#;
    fs::write(&path, custom).unwrap();

    let config = AppConfig::load(&path).unwrap();
    assert_eq!(config.nostr.relays, vec!["wss://custom.relay"]);
    assert_eq!(config.ui.theme, voter::config::Theme::Light);
}

#[test]
fn save_and_reload_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("voter.toml");

    let config = AppConfig::default();
    config.save(&path).unwrap();

    let loaded = AppConfig::load(&path).unwrap();
    assert_eq!(config.nostr.relays, loaded.nostr.relays);
}
