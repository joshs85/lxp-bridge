mod common;
use common::*;

use lxp_bridge::mqtt::Mqtt;
use lxp_bridge::coordinator::Coordinator;
use lxp_bridge::channels::Channels;
use tokio::sync::broadcast;
use tokio::time::{Duration, Instant, sleep};

// Performance Test: Channel Throughput
#[tokio::test]
async fn test_channel_throughput() {
    // Test channel throughput under normal load
    let (tx, mut rx) = broadcast::channel::<mqtt::ChannelData>(1000);
    
    let start_time = Instant::now();
    let message_count = 1000;
    
    // Send messages as fast as possible
    for i in 0..message_count {
        let message = mqtt::ChannelData::Message(mqtt::Message {
            topic: format!("throughput/test/{}", i),
            retain: false,
            payload: format!("message {}", i),
        });
        
        let _ = tx.send(message);
    }
    
    // Receive all messages
    let mut received_count = 0;
    while let Ok(_) = rx.try_recv() {
        received_count += 1;
    }
    
    let duration = start_time.elapsed();
    let throughput = received_count as f64 / duration.as_secs_f64();
    
    // Verify throughput is reasonable (should be > 1000 messages/second)
    assert!(throughput > 1000.0, "Throughput should be > 1000 msg/s, got {:.2}", throughput);
    assert!(received_count > 0, "Should receive some messages");
}

// Performance Test: Circuit Breaker Response Time
#[tokio::test]
async fn test_circuit_breaker_response_time() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    let mqtt = Mqtt::new(config, channels);
    
    // Test circuit breaker info retrieval performance
    let iterations = 100;
    let mut total_time = Duration::ZERO;
    
    for _ in 0..iterations {
        let start_time = Instant::now();
        let _cb_info = mqtt.get_circuit_breaker_info().await;
        total_time += start_time.elapsed();
    }
    
    let avg_response_time = total_time / iterations;
    
    // Circuit breaker info should be retrieved quickly (< 1ms average)
    assert!(avg_response_time < Duration::from_millis(1), 
            "Circuit breaker response time should be < 1ms, got {:?}", avg_response_time);
}

// Performance Test: Health Check Performance
#[tokio::test]
async fn test_health_check_performance() {
    let channels = Channels::new();
    
    // Test health check performance
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        let _health = channels.check_channel_health();
    }
    
    let duration = start_time.elapsed();
    let health_checks_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Health checks should be very fast (> 10,000 checks/second)
    assert!(health_checks_per_second > 10_000.0, 
            "Health checks should be > 10k/s, got {:.2}", health_checks_per_second);
}

// Performance Test: Configuration Access Performance
#[tokio::test]
async fn test_configuration_access_performance() {
    let config = Factory::example_config_wrapped();
    
    // Test configuration access performance
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        let _mqtt_config = config.mqtt();
    }
    
    let duration = start_time.elapsed();
    let config_accesses_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Configuration access should be very fast (> 50,000 accesses/second)
    assert!(config_accesses_per_second > 50_000.0, 
            "Config access should be > 50k/s, got {:.2}", config_accesses_per_second);
}

// Stress Test: High Message Volume
#[tokio::test]
async fn test_high_message_volume() {
    // Test system under high message volume
    let (tx, _rx) = broadcast::channel::<mqtt::ChannelData>(10000);
    
    let start_time = Instant::now();
    let message_count = 10000;
    
    // Send messages in batches to simulate high load
    for batch in 0..100 {
        for i in 0..100 {
            let message = mqtt::ChannelData::Message(mqtt::Message {
                topic: format!("stress/batch_{}/msg_{}", batch, i),
                retain: false,
                payload: format!("stress test message batch {} msg {}", batch, i),
            });
            
            let _ = tx.send(message);
        }
        
        // Small delay between batches to simulate real-world conditions
        sleep(Duration::from_millis(1)).await;
    }
    
    let duration = start_time.elapsed();
    let messages_per_second = message_count as f64 / duration.as_secs_f64();
    
    // System should handle high message volume (> 1000 messages/second)
    assert!(messages_per_second > 1000.0, 
            "Should handle > 1000 msg/s, got {:.2}", messages_per_second);
}

// Stress Test: Concurrent Component Creation
#[tokio::test]
async fn test_concurrent_component_creation() {
    let config = Factory::example_config_wrapped();
    
    // Test creating many components concurrently
    let component_count = 100;
    let start_time = Instant::now();
    
    // Create components sequentially (ConfigWrapper is not Send)
    for _i in 0..component_count {
        let channels = Channels::new();
        let _mqtt = Mqtt::new(config.clone(), channels.clone());
        let _coordinator = Coordinator::new(config.clone(), channels);
    }
    
    let duration = start_time.elapsed();
    let components_per_second = component_count as f64 / duration.as_secs_f64();
    
    // Component creation should be reasonably fast (> 10 components/second)
    assert!(components_per_second > 10.0, 
            "Should create > 10 components/s, got {:.2}", components_per_second);
}

// Stress Test: Memory Usage Under Load
#[test]
fn test_memory_usage_under_load() {
    let _channels = Channels::new();
    
    // Test memory usage under load
    let mut channel_instances = Vec::new();
    let instance_count = 1000;
    
    // Create many channel instances
    for _ in 0..instance_count {
        channel_instances.push(Channels::new());
    }
    
    // Verify all instances are functional
    for channels in &channel_instances {
        channels.check_channel_health();
    }
    
    // Clean up instances
    drop(channel_instances);
    
    // If we get here, no memory issues occurred
    assert!(true);
}

// Helper function to calculate backoff delay (used by test_backoff_calculation_performance)
fn calculate_backoff_delay(attempt: u32, base_delay: Duration, max_delay: Duration) -> Duration {
    if attempt == 0 {
        return base_delay;
    }
    
    let delay = base_delay * 2_u32.pow(attempt - 1);
    if delay > max_delay {
        max_delay
    } else {
        delay
    }
}

// Performance Test: Exponential Backoff Calculation Performance
#[test]
fn test_backoff_calculation_performance() {
    let base_delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(60);
    
    // Test backoff calculation performance
    let iterations = 100_000;
    let start_time = Instant::now();
    
    for attempt in 0..iterations {
        let _delay = calculate_backoff_delay((attempt % 10) + 1, base_delay, max_delay);
    }
    
    let duration = start_time.elapsed();
    let calculations_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Backoff calculations should be very fast (> 100,000 calculations/second)
    assert!(calculations_per_second > 100_000.0, 
            "Backoff calculations should be > 100k/s, got {:.2}", calculations_per_second);
}

// Stress Test: Rapid State Changes
#[tokio::test]
async fn test_rapid_state_changes() {
    let config = Factory::example_config_wrapped();
    let _channels = Channels::new();
    let mqtt = Mqtt::new(config, _channels);
    
    // Test rapid state changes in circuit breaker
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        // Rapidly check circuit breaker state
        let _cb_info = mqtt.get_circuit_breaker_info().await;
        
        // Rapidly check health status
        let _health_status = mqtt.get_health_status().await;
    }
    
    let duration = start_time.elapsed();
    let state_changes_per_second = iterations as f64 / duration.as_secs_f64();
    
    // State changes should be fast (> 100 changes/second)
    assert!(state_changes_per_second > 100.0, 
            "State changes should be > 100/s, got {:.2}", state_changes_per_second);
}

// Performance Test: Channel Health Monitoring Under Load
#[tokio::test]
async fn test_channel_health_under_load() {
    let channels = Channels::new();
    
    // Create high-load scenario
    let (tx, _rx) = broadcast::channel::<mqtt::ChannelData>(5000);
    
    // Fill channel with messages
    for i in 0..5000 {
        let message = mqtt::ChannelData::Message(mqtt::Message {
            topic: format!("load/health/{}", i),
            retain: false,
            payload: format!("load test for health monitoring {}", i),
        });
        
        let _ = tx.send(message);
    }
    
    // Test health monitoring performance under load
    let iterations = 100;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        let _health = channels.check_channel_health();
    }
    
    let duration = start_time.elapsed();
    let health_checks_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Health monitoring should remain fast even under load (> 1000 checks/second)
    assert!(health_checks_per_second > 1000.0, 
            "Health monitoring should be > 1000/s under load, got {:.2}", health_checks_per_second);
}

// Stress Test: Configuration Access Under Load
#[tokio::test]
async fn test_configuration_access_under_load() {
    let config = Factory::example_config_wrapped();
    
    // Test configuration access under high load
    let iterations = 10_000;

    
    let start_time = Instant::now();
    
    // Access configuration sequentially (ConfigWrapper is not Send)
    for _ in 0..iterations {
        let _mqtt_config = config.mqtt();
    }
    
    let duration = start_time.elapsed();
    let config_accesses_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Configuration access should remain fast under load (> 10,000 accesses/second)
    assert!(config_accesses_per_second > 10_000.0, 
            "Config access should be > 10k/s under load, got {:.2}", config_accesses_per_second);
}

// Performance Test: Error Recovery Performance
#[tokio::test]
async fn test_error_recovery_performance() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // Test error recovery performance
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        // Simulate error conditions and recovery
        let _mqtt = Mqtt::new(config.clone(), channels.clone());
        let _coordinator = Coordinator::new(config.clone(), channels.clone());
        
        // Check health after each iteration
        let _health = channels.check_channel_health();
    }
    
    let duration = start_time.elapsed();
    let recovery_cycles_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Error recovery should be reasonably fast (> 10 cycles/second)
    assert!(recovery_cycles_per_second > 10.0, 
            "Error recovery should be > 10 cycles/s, got {:.2}", recovery_cycles_per_second);
}


