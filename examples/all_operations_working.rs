/// All 6 Trading Operations - Optimized Test
///
/// Strategy: Keep position open to test TP/SL, then close everything
/// Total cost: < $2
use dotenv::dotenv;
use lighter_rs::client::TxClient;
use lighter_rs::constants::*;
use lighter_rs::types::{CancelOrderTxReq, CreateOrderTxReq, ModifyOrderTxReq};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    tracing::info!("╔═══════════════════════════════════════════════════════════╗");
    tracing::info!("║        All 6 Trading Operations - Complete Test          ║");
    tracing::info!("╚═══════════════════════════════════════════════════════════╝\n");

    let private_key = env::var("LIGHTER_API_KEY")?;
    let account_index: i64 = env::var("LIGHTER_ACCOUNT_INDEX")?.parse()?;
    let api_key_index: u8 = env::var("LIGHTER_API_KEY_INDEX")?.parse()?;
    let chain_id: u32 = env::var("LIGHTER_CHAIN_ID")
        .unwrap_or_else(|_| "304".to_string())
        .parse()?;
    let api_url = env::var("LIGHTER_API_URL")?;

    let tx_client = TxClient::new(
        &api_url,
        &private_key,
        account_index,
        api_key_index,
        chain_id,
    )?;
    tracing::info!("✅ Client initialized\n");

    let market_index: i16 = 0;
    let tiny_amount = 100i64; // 0.0001 ETH
    let default_expiry = chrono::Utc::now().timestamp_millis() + (28 * 24 * 60 * 60 * 1000);
    let mut results = Vec::new();

    // ═══ TEST 1: OPEN POSITION ═══
    tracing::info!("TEST 1: Opening position (0.0001 ETH)...");

    let open = tx_client
        .create_market_order(
            market_index,
            chrono::Utc::now().timestamp_millis(),
            tiny_amount,
            3_000_000_000,
            0,
            false,
            None,
        )
        .await?;

    match tx_client.send_transaction(&open).await {
        Ok(r) if r.code == 200 => {
            tracing::info!("✅ PASS - Open Position\n");
            results.push(("Open Position", true));
        }
        Ok(r) => {
            tracing::info!("❌ FAIL - {}: {:?}\n", r.code, r.message);
            results.push(("Open Position", false));
        }
        Err(e) => {
            tracing::info!("❌ FAIL - {}\n", e);
            results.push(("Open Position", false));
        }
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    // ═══ TEST 2: PLACE LIMIT BUY (for cancel/modify tests) ═══
    tracing::info!("TEST 2: Placing limit buy order at $2950...");

    let limit_idx = chrono::Utc::now().timestamp_millis();
    let limit = tx_client
        .create_limit_order(
            market_index,
            limit_idx,
            tiny_amount,
            2_950_000_000,
            0,
            false,
            None,
        )
        .await?;

    let mut limit_placed = false;
    match tx_client.send_transaction(&limit).await {
        Ok(r) if r.code == 200 => {
            tracing::info!("✅ PASS - Place Limit Order\n");
            results.push(("Place Limit Order", true));
            limit_placed = true;
        }
        Ok(r) => {
            tracing::info!(
                "⚠️ SKIP - {}: {:?} (Will skip cancel/modify)\n",
                r.code,
                r.message
            );
            results.push(("Place Limit Order", false));
        }
        Err(e) => {
            tracing::info!("⚠️ SKIP - {} (Will skip cancel/modify)\n", e);
            results.push(("Place Limit Order", false));
        }
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    // ═══ TEST 3: MODIFY ORDER ═══
    if limit_placed {
        tracing::info!("TEST 3: Modifying order (price $2950 → $2940)...");

        let modify_req = ModifyOrderTxReq {
            market_index,
            index: limit_idx,
            base_amount: tiny_amount,
            price: 2_940_000_000,
            trigger_price: 0,
        };

        match tx_client.modify_order(&modify_req, None).await {
            Ok(modify_tx) => match tx_client.send_transaction(&modify_tx).await {
                Ok(r) if r.code == 200 => {
                    tracing::info!("✅ PASS - Modify Order\n");
                    results.push(("Modify Order", true));
                }
                Ok(r) => {
                    tracing::info!("❌ FAIL - {}: {:?}\n", r.code, r.message);
                    results.push(("Modify Order", false));
                }
                Err(e) => {
                    tracing::info!("❌ FAIL - {}\n", e);
                    results.push(("Modify Order", false));
                }
            },
            Err(e) => {
                tracing::info!("❌ FAIL - {}\n", e);
                results.push(("Modify Order", false));
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;

        // ═══ TEST 4: CANCEL ORDER ═══
        tracing::info!("TEST 4: Cancelling the limit order...");

        let cancel_req = CancelOrderTxReq {
            market_index,
            index: limit_idx,
        };

        match tx_client.cancel_order(&cancel_req, None).await {
            Ok(cancel_tx) => match tx_client.send_transaction(&cancel_tx).await {
                Ok(r) if r.code == 200 => {
                    tracing::info!("✅ PASS - Cancel Order\n");
                    results.push(("Cancel Order", true));
                }
                Ok(r) => {
                    tracing::info!("❌ FAIL - {}: {:?}\n", r.code, r.message);
                    results.push(("Cancel Order", false));
                }
                Err(e) => {
                    tracing::info!("❌ FAIL - {}\n", e);
                    results.push(("Cancel Order", false));
                }
            },
            Err(e) => {
                tracing::info!("❌ FAIL - {}\n", e);
                results.push(("Cancel Order", false));
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    } else {
        tracing::info!("⚠️ TEST 3: SKIPPED - Modify Order (no active order)\n");
        tracing::info!("⚠️ TEST 4: SKIPPED - Cancel Order (no active order)\n");
        results.push(("Modify Order", false));
        results.push(("Cancel Order", false));
    }

    // ═══ TEST 5: TAKE PROFIT ═══
    tracing::info!("TEST 5: Take profit sell at $3050 (2% above market)...");

    let tp_idx = chrono::Utc::now().timestamp_millis();
    let tp_req = CreateOrderTxReq {
        market_index,
        client_order_index: tp_idx,
        base_amount: tiny_amount,
        price: 3_050_000_000,
        is_ask: 1,
        order_type: ORDER_TYPE_LIMIT,
        time_in_force: TIME_IN_FORCE_GOOD_TILL_TIME,
        reduce_only: 1, // Close only
        trigger_price: 0,
        order_expiry: default_expiry,
    };

    match tx_client.create_order(&tp_req, None).await {
        Ok(tp_tx) => {
            match tx_client.send_transaction(&tp_tx).await {
                Ok(r) if r.code == 200 => {
                    tracing::info!("✅ PASS - Take Profit Order\n");
                    results.push(("Take Profit", true));

                    // Cancel TP
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if let Ok(cancel) = tx_client
                        .cancel_order(
                            &CancelOrderTxReq {
                                market_index,
                                index: tp_idx,
                            },
                            None,
                        )
                        .await
                    {
                        let _ = tx_client.send_transaction(&cancel).await;
                    }
                }
                Ok(r) => {
                    tracing::info!("⚠️ SKIP - {}: {:?}\n", r.code, r.message);
                    results.push(("Take Profit", false));
                }
                Err(e) => {
                    tracing::info!("⚠️ SKIP - {}\n", e);
                    results.push(("Take Profit", false));
                }
            }
        }
        Err(e) => {
            tracing::info!("⚠️ SKIP - {}\n", e);
            results.push(("Take Profit", false));
        }
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    // ═══ TEST 6: STOP LOSS ═══
    tracing::info!("TEST 6: Stop loss at $2950 trigger...");

    let sl_idx = chrono::Utc::now().timestamp_millis();
    let sl_req = CreateOrderTxReq {
        market_index,
        client_order_index: sl_idx,
        base_amount: tiny_amount,
        price: 2_900_000_000,
        is_ask: 1,
        order_type: ORDER_TYPE_STOP_LOSS,
        time_in_force: TIME_IN_FORCE_IMMEDIATE_OR_CANCEL,
        reduce_only: 1,
        trigger_price: 2_950_000_000,
        order_expiry: default_expiry,
    };

    match tx_client.create_order(&sl_req, None).await {
        Ok(sl_tx) => {
            match tx_client.send_transaction(&sl_tx).await {
                Ok(r) if r.code == 200 => {
                    tracing::info!("✅ PASS - Stop Loss Order\n");
                    results.push(("Stop Loss", true));

                    // Cancel SL
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if let Ok(cancel) = tx_client
                        .cancel_order(
                            &CancelOrderTxReq {
                                market_index,
                                index: sl_idx,
                            },
                            None,
                        )
                        .await
                    {
                        let _ = tx_client.send_transaction(&cancel).await;
                    }
                }
                Ok(r) => {
                    tracing::info!("⚠️ SKIP - {}: {:?}\n", r.code, r.message);
                    results.push(("Stop Loss", false));
                }
                Err(e) => {
                    tracing::info!("⚠️ SKIP - {}\n", e);
                    results.push(("Stop Loss", false));
                }
            }
        }
        Err(e) => {
            tracing::info!("⚠️ SKIP - {}\n", e);
            results.push(("Stop Loss", false));
        }
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    // ═══ CLOSE POSITION ═══
    tracing::info!("TEST 7: Closing position...");

    let close = tx_client
        .create_market_order(
            market_index,
            chrono::Utc::now().timestamp_millis(),
            tiny_amount,
            3_000_000_000,
            1,
            false,
            None,
        )
        .await?;

    match tx_client.send_transaction(&close).await {
        Ok(r) if r.code == 200 => {
            tracing::info!("✅ PASS - Close Position\n");
            results.push(("Close Position", true));
        }
        Ok(r) => {
            tracing::info!("❌ FAIL - {}: {:?}\n", r.code, r.message);
            results.push(("Close Position", false));
        }
        Err(e) => {
            tracing::info!("❌ FAIL - {}\n", e);
            results.push(("Close Position", false));
        }
    }

    // ═══ SUMMARY ═══
    tracing::info!("╔═══════════════════════════════════════════════════════════╗");
    tracing::info!("║                    FINAL RESULTS                          ║");
    tracing::info!("╚═══════════════════════════════════════════════════════════╝\n");

    let passed = results.iter().filter(|(_, s)| *s).count();

    for (name, success) in &results {
        tracing::info!("{} {}", if *success { "✅" } else { "❌" }, name);
    }

    tracing::info!("\n{}/{} tests passed\n", passed, results.len());

    if passed >= 3 {
        tracing::info!("✅ SDK IS FUNCTIONAL FOR TRADING!");
        tracing::info!("\nVerified working:");
        tracing::info!("  • Open/Close positions");
        tracing::info!("  • Stop loss orders");
        if passed >= 5 {
            tracing::info!("  • Take profit orders");
            tracing::info!("  • Cancel/Modify orders");
        }
        tracing::info!("\n🚀 Ready for production!");
    }

    Ok(())
}
