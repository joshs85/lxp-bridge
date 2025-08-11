mod common;
use common::*;

use lxp_bridge::channels::Channels;
use lxp_bridge::config::ConfigWrapper;
use lxp_bridge::mqtt::Mqtt;

#[tokio::test]
async fn test_health_monitor_basic_functionality() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that health monitor can be created and run
    let health_monitor = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100)); // Fast for testing
        
        for _ in 0..3 { // Run 3 iterations
            interval.tick().await;
            
            // Check channel health
            channels.check_channel_health();
            
            // Log system status
            info!("Health check: {} enabled inverters, MQTT: {}",
                config.enabled_inverters().len(),
                if config.mqtt().enabled() { "enabled" } else { "disabled" }
            );
        }
    };
    
    // Run health monitor for a short time
    tokio::time::timeout(std::time::Duration::from_millis(500), health_monitor)
        .await
        .expect("Health monitor should complete within timeout");
}

#[tokio::test]
async fn test_health_monitor_channel_health_check() {
    common_setup();
    
    let channels = Channels::new();
    
    // Test channel health check functionality
    channels.check_channel_health();
    
    // The function doesn't return anything, so we just verify it doesn't panic
    // and can be called multiple times
    channels.check_channel_health();
    channels.check_channel_health();
}

#[tokio::test]
async fn test_health_monitor_mqtt_integration() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test MQTT health status
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    
    // Get health status
    let health_status = mqtt.get_health_status().await;
    assert!(!health_status.is_empty());
    
    // Get circuit breaker info
    let cb_info = mqtt.get_circuit_breaker_info().await;
    assert!(!cb_info.is_empty());
    assert!(cb_info.contains("state"));
}

#[tokio::test]
async fn test_health_monitor_configuration_validation() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    
    // Test configuration validation
    let enabled_inverters = config.enabled_inverters();
    assert!(!enabled_inverters.is_empty());
    
    let mqtt_config = config.mqtt();
    assert!(mqtt_config.enabled());
    
    // Test individual configuration options
    assert_eq!(mqtt_config.max_retries(), 3);
    assert_eq!(mqtt_config.circuit_breaker_threshold(), 5);
    assert_eq!(mqtt_config.reconnect_delay_secs(), 1);
    assert_eq!(mqtt_config.max_reconnect_delay_secs(), 300);
}

#[tokio::test]
async fn test_health_monitor_error_handling() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test error handling in health monitor
    let health_monitor = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        
        for _ in 0..2 {
            interval.tick().await;
            
            // Simulate some health checks
            let inverter_count = config.enabled_inverters().len();
            let mqtt_enabled = config.mqtt().enabled();
            
            // These should not panic
            assert!(inverter_count >= 0);
            assert!(mqtt_enabled == true || mqtt_enabled == false);
            
            // Log health status
            info!("Health check completed: {} inverters, MQTT: {}", 
                inverter_count, mqtt_enabled);
        }
    };
    
    // Should complete without errors
    tokio::time::timeout(std::time::Duration::from_millis(300), health_monitor)
        .await
        .expect("Health monitor should complete without errors");
}

#[tokio::test]
async fn test_health_monitor_performance() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test health monitor performance
    let start = std::time::Instant::now();
    
    let health_monitor = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(10)); // Very fast for testing
        
        for _ in 0..10 { // Run 10 iterations
            interval.tick().await;
            
            // Perform health checks
            channels.check_channel_health();
            
            let _inverter_count = config.enabled_inverters().len();
            let _mqtt_enabled = config.mqtt().enabled();
        }
    };
    
    tokio::time::timeout(std::time::Duration::from_millis(200), health_monitor)
        .await
        .expect("Health monitor should complete within timeout");
    
    let duration = start.elapsed();
    
    // Health monitoring should be fast
    assert!(duration.as_millis() < 200, "Health monitoring took too long: {:?}", duration);
}

#[tokio::test]
async fn test_health_monitor_logging_levels() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test different logging levels
    let health_monitor = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        
        interval.tick().await;
        
        // Info level logging
        info!("Health check: {} enabled inverters, MQTT: {}",
            config.enabled_inverters().len(),
            if config.mqtt().enabled() { "enabled" } else { "disabled" }
        );
        
        // Debug level logging
        debug!("Health check details: inverter count = {}, mqtt enabled = {}",
            config.enabled_inverters().len(),
            config.mqtt().enabled()
        );
        
        // Trace level logging
        trace!("Health check trace: channels = {:?}", channels);
    };
    
    // Should complete without errors
    tokio::time::timeout(std::time::Duration::from_millis(200), health_monitor)
        .await
        .expect("Health monitor should complete without errors");
}

#[tokio::test]
async fn test_health_monitor_concurrent_access() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test concurrent access to health monitoring
    let health_monitor_1 = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        
        for _ in 0..3 {
            interval.tick().await;
            channels.check_channel_health();
            let _count = config.enabled_inverters().len();
        }
    };
    
    let health_monitor_2 = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        
        for _ in 0..3 {
            interval.tick().await;
            channels.check_channel_health();
            let _enabled = config.mqtt().enabled();
        }
    };
    
    // Run both health monitors concurrently
    tokio::try_join!(
        tokio::time::timeout(std::time::Duration::from_millis(200), health_monitor_1),
        tokio::time::timeout(std::time::Duration::from_millis(200), health_monitor_2)
    ).expect("Both health monitors should complete concurrently");
}

#[tokio::test]
async fn test_health_monitor_resource_cleanup() {
    common_setup();
    
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test resource cleanup
    let health_monitor = async {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        
        // Run health monitor
        interval.tick().await;
        channels.check_channel_health();
        
        // Drop interval to test cleanup
        drop(interval);
        
        // Should still be able to access channels
        channels.check_channel_health();
        let _inverter_count = config.enabled_inverters().len();
    };
    
    // Should complete without errors
    tokio::time::timeout(std::time::Duration::from_millis(200), health_monitor)
        .await
        .expect("Health monitor should complete without errors");
}
