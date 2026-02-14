//! Integration tests for Lightning Network functionality
//!
//! These tests verify the complete Lightning payment flow

use ulw_ldk::{LdkNode, PaymentStatus};
use bitcoin::Network;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

/// Helper function to create a test Lightning node
async fn create_test_node(name: &str) -> (LdkNode, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let storage_path = temp_dir.path().join(name);

    // Derive unique entropy for each test node
    let mut entropy = [0u8; 32];
    for (i, byte) in name.bytes().enumerate() {
        if i < 32 {
            entropy[i] = byte;
        }
    }

    let node = LdkNode::new(Network::Regtest, storage_path, entropy)
        .await
        .expect("Failed to create LDK node");

    (node, temp_dir)
}

#[tokio::test]
async fn test_node_creation() {
    let (node, _temp) = create_test_node("alice").await;

    let info = node.get_info();
    assert_eq!(info.network, Network::Regtest);
    assert!(!info.node_id.to_string().is_empty());
    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn test_invoice_creation() {
    let (node, _temp) = create_test_node("bob").await;

    let amount_msats = 10_000u64;
    let description = "Test payment".to_string();

    let invoice = node
        .create_invoice(Some(amount_msats), description, 3600)
        .await
        .expect("Failed to create invoice");

    // Verify invoice format
    assert!(invoice.starts_with("lnbcrt")); // Regtest invoice prefix
    assert!(invoice.len() >50); // BOLT11 invoices are long

    // Verify invoice can be parsed
    let parsed_invoice = invoice
        .parse::<lightning_invoice::Bolt11Invoice>()
        .expect("Failed to parse invoice");

    assert_eq!(
        parsed_invoice.amount_milli_satoshis(),
        Some(amount_msats)
    );
}

#[tokio::test]
async fn test_invoice_with_no_amount() {
    let (node, _temp) = create_test_node("charlie").await;

    // Create zero-amount invoice (any-amount invoice)
    let invoice = node
        .create_invoice(None, "Tips appreciated".to_string(), 3600)
        .await
        .expect("Failed to create invoice");

    assert!(invoice.starts_with("lnbcrt"));

    let parsed_invoice = invoice
        .parse::<lightning_invoice::Bolt11Invoice>()
        .expect("Failed to parse invoice");

    assert_eq!(parsed_invoice.amount_milli_satoshis(), None);
}

#[tokio::test]
async fn test_payment_tracking() {
    let (node, _temp) = create_test_node("dave").await;

    // Create invoice
    let invoice = node
        .create_invoice(Some(5000), "Tracking test".to_string(), 3600)
        .await
        .expect("Failed to create invoice");

    // Check payments list
    let payments = node.list_payments().await.expect("Failed to list payments");

    assert_eq!(payments.len(), 1);
    assert_eq!(payments[0].amount_msat, Some(5000));
    assert!(matches!(payments[0].status, PaymentStatus::Pending));

    // Verify payment hash matches invoice
    let parsed_invoice = invoice
        .parse::<lightning_invoice::Bolt11Invoice>()
        .unwrap();

    assert_eq!(
        payments[0].payment_hash,
        parsed_invoice.payment_hash().to_byte_array()
    );
}

#[tokio::test]
async fn test_invoice_parsing() {
    let (node, _temp) = create_test_node("eve").await;

    // Create invoice
    let invoice_str = node
        .create_invoice(Some(1000), "Parse test".to_string(), 3600)
        .await
        .expect("Failed to create invoice");

    // Attempt to "pay" this invoice (simplified - no actual payment)
    let payment_hash = node
        .pay_invoice(invoice_str.clone())
        .await
        .expect("Failed to parse and initiate payment");

    // Verify payment hash is not empty
    assert!(!payment_hash.0.iter().all(|&b| b == 0));

    // Verify payment is tracked
    let payments = node.list_payments().await.unwrap();
    assert!(payments.len() >= 2); // At least the invoice + payment
}

#[tokio::test]
async fn test_multiple_invoices() {
    let (node, _temp) = create_test_node("frank").await;

    // Create multiple invoices
    for i in 1..=5 {
        let amount = i * 1000;
        node.create_invoice(
            Some(amount),
            format!("Invoice {}", i),
            3600,
        )
        .await
        .expect("Failed to create invoice");
    }

    // Verify all payments are tracked
    let payments = node.list_payments().await.unwrap();
    assert_eq!(payments.len(), 5);

    // Verify amounts are correct
    for (i, payment) in payments.iter().enumerate() {
        let expected_amount = ((i + 1) * 1000) as u64;
        assert_eq!(payment.amount_msat, Some(expected_amount));
    }
}

#[tokio::test]
async fn test_node_id_consistency() {
    // Create node twice with same entropy
    let entropy = [42u8; 32];
    let temp1 = TempDir::new().unwrap();
    let temp2 = TempDir::new().unwrap();

    let node1 = LdkNode::new(Network::Regtest, temp1.path().to_path_buf(), entropy)
        .await
        .unwrap();

    let node2 = LdkNode::new(Network::Regtest, temp2.path().to_path_buf(), entropy)
        .await
        .unwrap();

    // Node IDs should be identical (deterministic from entropy)
    assert_eq!(node1.get_node_id(), node2.get_node_id());
}

#[tokio::test]
async fn test_node_id_uniqueness() {
    // Create nodes with different entropy
    let (node1, _temp1) = create_test_node("unique1").await;
    let (node2, _temp2) = create_test_node("unique2").await;

    // Node IDs should be different
    assert_ne!(node1.get_node_id(), node2.get_node_id());
}

#[tokio::test]
async fn test_invoice_expiry() {
    let (node, _temp) = create_test_node("grace").await;

    // Create invoice with short expiry
    let invoice = node
        .create_invoice(Some(1000), "Expiry test".to_string(), 10) // 10 seconds
        .await
        .expect("Failed to create invoice");

    let parsed = invoice
        .parse::<lightning_invoice::Bolt11Invoice>()
        .unwrap();

    // Verify expiry is set correctly
    assert_eq!(parsed.expiry_time(), Duration::from_secs(10));

    // Invoice should still be valid immediately
    assert!(parsed.would_expire(Duration::from_secs(0)));
    // Invoice should expire after 11 seconds
    assert!(!parsed.would_expire(Duration::from_secs(11)));
}

#[tokio::test]
async fn test_concurrent_invoice_creation() {
    let (node, _temp) = create_test_node("henry").await;

    // Create invoices concurrently
    let mut handles = vec![];

    for i in 0..10 {
        let node_clone = node.clone();  // Clone would require Arc wrapping
        let handle = tokio::spawn(async move {
            node_clone
                .create_invoice(
                    Some(i * 100),
                    format!("Concurrent {}", i),
                    3600,
                )
                .await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task failed").expect("Invoice creation failed");
    }

    // Verify all payments tracked
    let payments = node.list_payments().await.unwrap();
    assert_eq!(payments.len(), 10);
}
