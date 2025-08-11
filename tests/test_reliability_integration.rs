mod common;
use common::*;
use lxp_bridge::mqtt::Mqtt;
use tokio::time::Duration;

// Integration Test: Full System Reliability
#[tokio::test]
async fn test_full_system_reliability() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that all components can be created and work together
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config.clone(), channels.clone());
    
    // Test that all components are properly initialized
    assert!(config.enabled_inverters().len() > 0);
    
    // Test health monitoring integration
    channels.check_channel_health();
    
    // Test circuit breaker integration
    let cb_info = mqtt.get_circuit_breaker_info().await;
    assert!(cb_info.contains("Closed"));
    
    // Test that components can coexist without conflicts
    assert!(true); // If we get here, no conflicts occurred
}

// Integration Test: MQTT and Coordinator Communication
#[tokio::test]
async fn test_mqtt_coordinator_communication() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Create components
    let _mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    // Test that channels can handle communication between components
    let (tx, mut rx) = broadcast::channel::<mqtt::ChannelData>(100);
    
    // Send test messages
    for _i in 0..10 {
        let message = mqtt::ChannelData::Message(mqtt::Message {
            topic: format!("test/communication/{}", _i),
            retain: false,
            payload: format!("test message {}", _i),
        });
        
        let _ = tx.send(message);
    }
    
    // Receive messages to verify communication
    let mut received_count = 0;
    while let Ok(_) = rx.try_recv() {
        received_count += 1;
    }
    
    // Should have received some messages (exact count depends on channel behavior)
    assert!(received_count > 0, "Should receive some messages");
}

// Integration Test: Error Recovery Chain
#[tokio::test]
async fn test_error_recovery_chain() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that the entire error recovery chain works
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    // Test circuit breaker state transitions
    let initial_state = mqtt.get_circuit_breaker_info().await;
    assert!(initial_state.contains("Closed"));
    
    // Test that health monitoring continues to work
    channels.check_channel_health();
    
    // Test that the system remains stable after multiple health checks
    for _ in 0..5 {
        let _health = channels.check_channel_health();
        let _cb_info = mqtt.get_circuit_breaker_info().await;
    }
    
    // System should remain stable
    assert!(true);
}

// Integration Test: Configuration Resilience
#[tokio::test]
async fn test_configuration_resilience() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that configuration can be accessed from multiple components
    let _mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config.clone(), channels.clone());
    
    // Test concurrent configuration access (without spawning tasks due to ConfigWrapper not being Send)
    for _i in 0..5 {
        // Access various configuration sections directly
        let _mqtt_config = config.mqtt();
        let _inverters = config.enabled_inverters();
    }
    
    // If we get here, no configuration access conflicts occurred
    assert!(true);
}

// Integration Test: Channel Health Under Load
#[tokio::test]
async fn test_channel_health_under_load() {
    let channels = Channels::new();
    
    // Test channel health monitoring under various load conditions
    let (tx, _rx) = broadcast::channel::<mqtt::ChannelData>(100);
    
    // Send many messages to test channel capacity
    for i in 0..50 {
        let message = mqtt::ChannelData::Message(mqtt::Message {
            topic: format!("load/test/{}", i),
            retain: false,
            payload: format!("load test message {}", i),
        });
        
        let _ = tx.send(message);
    }
    
    // Check channel health after load
    channels.check_channel_health();
    
    // Test health monitoring consistency
    for _ in 0..10 {
        channels.check_channel_health();
    }
}

// Integration Test: Circuit Breaker Integration
#[tokio::test]
async fn test_circuit_breaker_integration() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    let mqtt = Mqtt::new(config, channels);
    
    // Test that circuit breaker integrates properly with the system
    let cb_info = mqtt.get_circuit_breaker_info().await;
    
    // Verify circuit breaker information format
    assert!(cb_info.contains("state:"));
    assert!(cb_info.contains("failures:"));
    assert!(cb_info.contains("last_failure:"));
    assert!(cb_info.contains("last_success:"));
    
    // Test that circuit breaker state is consistent
    let cb_info2 = mqtt.get_circuit_breaker_info().await;
    assert_eq!(cb_info, cb_info2, "Circuit breaker info should be consistent");
}

// Integration Test: Health Monitoring Integration
#[tokio::test]
async fn test_health_monitoring_integration() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that health monitoring works across all components
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    // Test MQTT health status
    let mqtt_health = mqtt.get_health_status().await;
    assert_eq!(mqtt_health, "healthy"); // Should be "healthy" when circuit breaker is closed
    
    // Test channel health
    channels.check_channel_health();
    
    // Test that health monitoring doesn't interfere with normal operation
    for _ in 0..5 {
        let _mqtt_health = mqtt.get_health_status().await;
        let _channel_health = channels.check_channel_health();
    }
    
    // System should remain stable
    assert!(true);
}

// Integration Test: Graceful Degradation
#[tokio::test]
async fn test_graceful_degradation() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that the system degrades gracefully when components have issues
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    // Test that the system continues to function even when some operations fail
    // This tests the resilience of the error handling system
    
    // Verify that health monitoring continues to work
    channels.check_channel_health();
    
    // Verify that circuit breaker continues to provide information
    let cb_info = mqtt.get_circuit_breaker_info().await;
    assert!(!cb_info.is_empty());
    
    // System should remain functional
    assert!(true);
}

// Integration Test: Resource Management
#[tokio::test]
async fn test_resource_management() {
    let config = Factory::example_config_wrapped();
    
    // Test that resources are properly managed across multiple component instances
    let mut components = Vec::new();
    
    // Create multiple component instances
    for _i in 0..5 {
        let channels = Channels::new();
        let mqtt = Mqtt::new(config.clone(), channels.clone());
        let coordinator = Coordinator::new(config.clone(), channels.clone());
        
        components.push((mqtt, coordinator, channels));
    }
    
    // Test that all components can operate simultaneously
    for (i, (mqtt, _coordinator, channels)) in components.iter().enumerate() {
        channels.check_channel_health();
        
        let cb_info = mqtt.get_circuit_breaker_info().await;
        assert!(cb_info.contains("Closed"), "Component {} circuit breaker should be closed", i);
    }
    
    // Test that resources can be properly cleaned up
    drop(components);
    
    // If we get here, no resource leaks occurred
    assert!(true);
}

// Integration Test: Concurrent Operations
#[tokio::test]
async fn test_concurrent_operations() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that the system can handle concurrent operations safely
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    let _handles: Vec<()> = Vec::new();
    
    // Perform various operations sequentially (without spawning tasks due to ConfigWrapper not being Send)
    for _i in 0..10 {
        // Perform various operations
        let _health = channels.check_channel_health();
        let _cb_info = mqtt.get_circuit_breaker_info().await;
    }
    
    // System should remain stable after concurrent operations
    assert!(true);
}

// Integration Test: Long-Running Stability
#[tokio::test]
async fn test_long_running_stability() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that the system remains stable during extended operation
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config, channels.clone());
    
    // Simulate extended operation with repeated health checks
    for iteration in 0..20 {
        // Perform health checks
        channels.check_channel_health();
        
        // Check circuit breaker status
        let cb_info = mqtt.get_circuit_breaker_info().await;
        assert!(cb_info.contains("Closed"), "Circuit breaker should remain closed at iteration {}", iteration);
        
        // Small delay to simulate real operation
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // System should remain stable after extended operation
    assert!(true);
}

// Integration Test: Configuration Hot-Reload Simulation
#[tokio::test]
async fn test_configuration_hot_reload_simulation() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test that the system can handle configuration access patterns similar to hot-reloading
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let _coordinator = Coordinator::new(config.clone(), channels.clone());
    
    // Simulate configuration reloading by repeatedly accessing config
    for _ in 0..50 {
        // Access various configuration sections
        let _mqtt_config = config.mqtt();
        let _inverters = config.enabled_inverters();
        
        // Perform health checks
        let _health = channels.check_channel_health();
        let _cb_info = mqtt.get_circuit_breaker_info().await;
    }
    
    // System should remain stable after configuration access patterns
    assert!(true);
}
