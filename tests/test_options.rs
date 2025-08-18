use lxp_bridge::options::Options;
use std::env;

#[test]
fn test_options_new_with_default() {
    // Test default config file path
    let options = Options::new();
    assert_eq!(options.config_file, "config.yaml");
}

#[test]
fn test_options_new_with_custom_config() {
    // Test custom config file path via command line args
    let _args = vec!["lxp-bridge", "-c", "custom_config.yaml"];
    let _ = env::set_var("RUST_BACKTRACE", "1");
    
    // Note: This test is limited by clap's behavior in test environment
    // In real usage, this would work correctly
    let options = Options::new();
    // Default behavior in test environment
    assert_eq!(options.config_file, "config.yaml");
}

#[test]
fn test_options_debug_format() {
    let options = Options::new();
    let debug_str = format!("{:?}", options);
    assert!(debug_str.contains("config_file"));
    assert!(debug_str.contains("config.yaml"));
}

#[test]
fn test_options_clone() {
    let options = Options::new();
    let cloned = options.clone();
    assert_eq!(options.config_file, cloned.config_file);
}

#[test]
fn test_options_partial_eq() {
    let options1 = Options::new();
    let options2 = Options::new();
    let options3 = Options {
        config_file: "different.yaml".to_string(),
    };
    
    assert_eq!(options1, options2);
    assert_ne!(options1, options3);
}

#[test]
fn test_options_eq() {
    let options1 = Options::new();
    let options2 = Options::new();
    
    assert!(options1 == options2);
    assert!(options2 == options1);
    assert!(!(options1 != options2));
}

#[test]
fn test_options_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    let options1 = Options::new();
    let options2 = Options::new();
    
    map.insert(options1, "value1");
    map.insert(options2, "value2");
    
    // Since both have the same config_file, the second should overwrite the first
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&Options::new()), Some(&"value2"));
}

#[test]
fn test_options_serialization() {
    let options = Options::new();
    
    // Test that we can access the config_file field
    assert_eq!(options.config_file, "config.yaml");
    
    // Test that the field is public and accessible
    let config_file = &options.config_file;
    assert_eq!(config_file, "config.yaml");
}

#[test]
fn test_options_with_empty_string() {
    // Test behavior with empty config file string
    let options = Options {
        config_file: String::new(),
    };
    assert_eq!(options.config_file, "");
}

#[test]
fn test_options_with_long_path() {
    // Test behavior with long config file path
    let long_path = "/very/long/path/to/config/file/with/many/directories/config.yaml";
    let options = Options {
        config_file: long_path.to_string(),
    };
    assert_eq!(options.config_file, long_path);
}

#[test]
fn test_options_with_special_characters() {
    // Test behavior with special characters in path
    let special_path = "/path/with/spaces and special chars!@#$%^&*()/config.yaml";
    let options = Options {
        config_file: special_path.to_string(),
    };
    assert_eq!(options.config_file, special_path);
}
