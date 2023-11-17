use std::sync::Arc;

use anyhow::Result;
use breez_sdk_core::*;

async fn list_supported_fiat_currencies(sdk: Arc<BreezServices>) -> Result<()> {
    // ANCHOR: list-fiat-currencies
    let supported_fiat_currencies = sdk.list_fiat_currencies().await?;
    // ANCHOR_END: list-fiat-currencies

    Ok(())
}

async fn get_current_rates(sdk: Arc<BreezServices>) -> Result<()> {
    // ANCHOR: fetch-fiat-rates
    let fiat_rates = sdk.fetch_fiat_rates().await?;
    // ANCHOR_END: fetch-fiat-rates

    Ok(())
}
