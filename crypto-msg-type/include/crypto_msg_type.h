/* Licensed under Apache-2.0 */
#ifndef CRYPTO_MSG_TYPE_H_
#define CRYPTO_MSG_TYPE_H_

/**
 * Crypto message types.
 *
 * L2Snapshot and L2TopK are very similar, the former is from RESTful API,
 * the latter is from websocket.
 */
typedef enum MessageType {
  /**
   * All other messages
   */
  Other,
  /**
   * tick-by-tick trade messages
   */
  Trade,
  /**
   * Incremental level2 orderbook updates
   */
  L2Event,
  /**
   * Level2 snapshot from RESTful API
   */
  L2Snapshot,
  /**
   * Level2 top K snapshots from websocket
   */
  L2TopK,
  /**
   * Incremental level3 orderbook updates
   */
  L3Event,
  /**
   * Level3 snapshot from RESTful API
   */
  L3Snapshot,
  /**
   * Best bid and ask
   */
  BBO,
  /**
   * 24hr rolling window ticker
   */
  Ticker,
  /**
   * OHLCV candlestick
   */
  Candlestick,
  /**
   * Funding rate
   */
  FundingRate,
  /**
   * Open interest
   */
  OpenInterest,
} MessageType;

#endif /* CRYPTO_MSG_TYPE_H_ */
