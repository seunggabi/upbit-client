/* 
 * Upbit Open API
 *
 * ## REST API for Upbit Exchange - Base URL: [https://api.upbit.com] - Official Upbit API Documents: [https://docs.upbit.com] - Official Support email: [open-api@upbit.com] 
 *
 * OpenAPI spec version: 1.0.0
 * Contact: ujhin942@gmail.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleMinutes {
  /// 마켓명
  #[serde(rename = "market")]
  market: Option<String>,
  /// 캔들 기준 시각 (UTC 기준)
  #[serde(rename = "candle_date_time_utc")]
  candle_date_time_utc: Option<String>,
  /// 캔들 기준 시각 (KST 기준)
  #[serde(rename = "candle_date_time_kst")]
  candle_date_time_kst: Option<String>,
  /// 시가
  #[serde(rename = "opening_price")]
  opening_price: Option<f64>,
  /// 고가
  #[serde(rename = "high_price")]
  high_price: Option<f64>,
  /// 저가
  #[serde(rename = "low_price")]
  low_price: Option<f64>,
  /// 종가
  #[serde(rename = "trade_price")]
  trade_price: Option<f64>,
  /// 해당 캔들에서 마지막 틱이 저장된 시각
  #[serde(rename = "timestamp")]
  timestamp: Option<f32>,
  /// 누적 거래 금액
  #[serde(rename = "candle_acc_trade_price")]
  candle_acc_trade_price: Option<f64>,
  /// 누적 거래량
  #[serde(rename = "candle_acc_trade_volume")]
  candle_acc_trade_volume: Option<f64>,
  /// 분 단위(유닛)
  #[serde(rename = "unit")]
  unit: Option<f32>
}

impl CandleMinutes {
  pub fn new() -> CandleMinutes {
    CandleMinutes {
      market: None,
      candle_date_time_utc: None,
      candle_date_time_kst: None,
      opening_price: None,
      high_price: None,
      low_price: None,
      trade_price: None,
      timestamp: None,
      candle_acc_trade_price: None,
      candle_acc_trade_volume: None,
      unit: None
    }
  }

  pub fn set_market(&mut self, market: String) {
    self.market = Some(market);
  }

  pub fn with_market(mut self, market: String) -> CandleMinutes {
    self.market = Some(market);
    self
  }

  pub fn market(&self) -> Option<&String> {
    self.market.as_ref()
  }

  pub fn reset_market(&mut self) {
    self.market = None;
  }

  pub fn set_candle_date_time_utc(&mut self, candle_date_time_utc: String) {
    self.candle_date_time_utc = Some(candle_date_time_utc);
  }

  pub fn with_candle_date_time_utc(mut self, candle_date_time_utc: String) -> CandleMinutes {
    self.candle_date_time_utc = Some(candle_date_time_utc);
    self
  }

  pub fn candle_date_time_utc(&self) -> Option<&String> {
    self.candle_date_time_utc.as_ref()
  }

  pub fn reset_candle_date_time_utc(&mut self) {
    self.candle_date_time_utc = None;
  }

  pub fn set_candle_date_time_kst(&mut self, candle_date_time_kst: String) {
    self.candle_date_time_kst = Some(candle_date_time_kst);
  }

  pub fn with_candle_date_time_kst(mut self, candle_date_time_kst: String) -> CandleMinutes {
    self.candle_date_time_kst = Some(candle_date_time_kst);
    self
  }

  pub fn candle_date_time_kst(&self) -> Option<&String> {
    self.candle_date_time_kst.as_ref()
  }

  pub fn reset_candle_date_time_kst(&mut self) {
    self.candle_date_time_kst = None;
  }

  pub fn set_opening_price(&mut self, opening_price: f64) {
    self.opening_price = Some(opening_price);
  }

  pub fn with_opening_price(mut self, opening_price: f64) -> CandleMinutes {
    self.opening_price = Some(opening_price);
    self
  }

  pub fn opening_price(&self) -> Option<&f64> {
    self.opening_price.as_ref()
  }

  pub fn reset_opening_price(&mut self) {
    self.opening_price = None;
  }

  pub fn set_high_price(&mut self, high_price: f64) {
    self.high_price = Some(high_price);
  }

  pub fn with_high_price(mut self, high_price: f64) -> CandleMinutes {
    self.high_price = Some(high_price);
    self
  }

  pub fn high_price(&self) -> Option<&f64> {
    self.high_price.as_ref()
  }

  pub fn reset_high_price(&mut self) {
    self.high_price = None;
  }

  pub fn set_low_price(&mut self, low_price: f64) {
    self.low_price = Some(low_price);
  }

  pub fn with_low_price(mut self, low_price: f64) -> CandleMinutes {
    self.low_price = Some(low_price);
    self
  }

  pub fn low_price(&self) -> Option<&f64> {
    self.low_price.as_ref()
  }

  pub fn reset_low_price(&mut self) {
    self.low_price = None;
  }

  pub fn set_trade_price(&mut self, trade_price: f64) {
    self.trade_price = Some(trade_price);
  }

  pub fn with_trade_price(mut self, trade_price: f64) -> CandleMinutes {
    self.trade_price = Some(trade_price);
    self
  }

  pub fn trade_price(&self) -> Option<&f64> {
    self.trade_price.as_ref()
  }

  pub fn reset_trade_price(&mut self) {
    self.trade_price = None;
  }

  pub fn set_timestamp(&mut self, timestamp: f32) {
    self.timestamp = Some(timestamp);
  }

  pub fn with_timestamp(mut self, timestamp: f32) -> CandleMinutes {
    self.timestamp = Some(timestamp);
    self
  }

  pub fn timestamp(&self) -> Option<&f32> {
    self.timestamp.as_ref()
  }

  pub fn reset_timestamp(&mut self) {
    self.timestamp = None;
  }

  pub fn set_candle_acc_trade_price(&mut self, candle_acc_trade_price: f64) {
    self.candle_acc_trade_price = Some(candle_acc_trade_price);
  }

  pub fn with_candle_acc_trade_price(mut self, candle_acc_trade_price: f64) -> CandleMinutes {
    self.candle_acc_trade_price = Some(candle_acc_trade_price);
    self
  }

  pub fn candle_acc_trade_price(&self) -> Option<&f64> {
    self.candle_acc_trade_price.as_ref()
  }

  pub fn reset_candle_acc_trade_price(&mut self) {
    self.candle_acc_trade_price = None;
  }

  pub fn set_candle_acc_trade_volume(&mut self, candle_acc_trade_volume: f64) {
    self.candle_acc_trade_volume = Some(candle_acc_trade_volume);
  }

  pub fn with_candle_acc_trade_volume(mut self, candle_acc_trade_volume: f64) -> CandleMinutes {
    self.candle_acc_trade_volume = Some(candle_acc_trade_volume);
    self
  }

  pub fn candle_acc_trade_volume(&self) -> Option<&f64> {
    self.candle_acc_trade_volume.as_ref()
  }

  pub fn reset_candle_acc_trade_volume(&mut self) {
    self.candle_acc_trade_volume = None;
  }

  pub fn set_unit(&mut self, unit: f32) {
    self.unit = Some(unit);
  }

  pub fn with_unit(mut self, unit: f32) -> CandleMinutes {
    self.unit = Some(unit);
    self
  }

  pub fn unit(&self) -> Option<&f32> {
    self.unit.as_ref()
  }

  pub fn reset_unit(&mut self) {
    self.unit = None;
  }

}



