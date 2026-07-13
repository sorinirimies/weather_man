use weatherman_core::AppSettings;

#[test]
fn add_location_dedups_case_insensitively() {
    let mut s = AppSettings::default();
    assert!(s.add_location("Berlin"));
    assert!(!s.add_location("berlin")); // duplicate (case-insensitive)
    assert!(!s.add_location("  BERLIN ")); // duplicate after trim
    assert!(s.add_location("Paris"));
    assert_eq!(s.locations, vec!["Berlin".to_string(), "Paris".to_string()]);
}

#[test]
fn add_location_rejects_empty() {
    let mut s = AppSettings::default();
    assert!(!s.add_location(""));
    assert!(!s.add_location("   "));
    assert!(s.locations.is_empty());
}

#[test]
fn remove_location_by_index() {
    let mut s = AppSettings::default();
    s.add_location("Berlin");
    s.add_location("Paris");
    s.add_location("Tokyo");
    s.remove_location(1); // remove Paris
    assert_eq!(s.locations, vec!["Berlin".to_string(), "Tokyo".to_string()]);
    s.remove_location(99); // out of range -> no-op
    assert_eq!(s.locations.len(), 2);
}

#[test]
fn default_units_is_metric() {
    assert_eq!(AppSettings::default().units, "metric");
}

#[test]
fn settings_roundtrip_json() {
    let s = AppSettings {
        units: "imperial".to_string(),
        locations: vec!["Berlin".to_string()],
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: AppSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(back.units, "imperial");
    assert_eq!(back.locations, vec!["Berlin".to_string()]);
}

#[test]
fn deserializes_partial_json_with_defaults() {
    // Missing fields fall back to defaults.
    let back: AppSettings = serde_json::from_str("{}").unwrap();
    assert_eq!(back.units, "metric");
    assert!(back.locations.is_empty());
}
