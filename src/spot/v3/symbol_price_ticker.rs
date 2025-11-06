use crate::spot::v3::{ApiResponse, ApiResult};
use crate::spot::MexcSpotApiTrait;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTickerParams<'a> {
    /// If omitted, returns prices for all symbols
    pub symbol: Option<&'a str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: Decimal,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
enum SymbolPriceTickerResponse {
    Single(SymbolPriceTicker),
    Many(Vec<SymbolPriceTicker>),
}

#[async_trait]
pub trait SymbolPriceTickerEndpoint {
    /// Symbol Price Ticker
    async fn symbol_price_ticker(
        &self,
        params: SymbolPriceTickerParams<'_>,
    ) -> ApiResult<Vec<SymbolPriceTicker>>;
}

#[async_trait]
impl<T: MexcSpotApiTrait + Sync> SymbolPriceTickerEndpoint for T {
    async fn symbol_price_ticker(
        &self,
        params: SymbolPriceTickerParams<'_>,
    ) -> ApiResult<Vec<SymbolPriceTicker>> {
        let endpoint = format!("{}/api/v3/ticker/price", self.endpoint().as_ref());
        let response = self
            .reqwest_client()
            .get(&endpoint)
            .query(&params)
            .send()
            .await?;
        let api_response = response
            .json::<ApiResponse<SymbolPriceTickerResponse>>()
            .await?;
        let output = api_response.into_api_result()?;
        let tickers = match output {
            SymbolPriceTickerResponse::Single(t) => vec![t],
            SymbolPriceTickerResponse::Many(list) => list,
        };
        Ok(tickers)
    }
}

#[cfg(test)]
mod tests {
    use crate::spot::MexcSpotApiClient;

    use super::*;

    #[tokio::test]
    async fn test_symbol_price_ticker_single() {
        let client = MexcSpotApiClient::default();
        let params = SymbolPriceTickerParams { symbol: Some("BTCUSDT") };
        let result = client.symbol_price_ticker(params).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].symbol, "BTCUSDT");
    }

    #[tokio::test]
    async fn test_symbol_price_ticker_all() {
        let client = MexcSpotApiClient::default();
        let params = SymbolPriceTickerParams { symbol: None };
        let result = client.symbol_price_ticker(params).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert!(!list.is_empty());
    }
}


