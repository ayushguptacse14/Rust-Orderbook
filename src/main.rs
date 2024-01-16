mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order};
use rust_decimal_macros::dec;

fn main() {
    // Initialize the orderbook and matching engine
    let mut engine = MatchingEngine::new();


    // Print current state of the orderbook
    // println!("Orderbook before matching engine:\n{:?} \n", orderbook);


    // Create a trading pair
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

    // Add the market to the matching engine
    engine.add_new_market(pair.clone());

    //Show state of Orderbook
    engine.show_orderbook_state();

    // Place a limit order using the matching engine
    let buy_order_from_ayush = Order::new(BidOrAsk::Bid, 7.5);
    let buy_order_from_sameep = Order::new(BidOrAsk::Bid, 20.5);
    let sell_order_from_ayush = Order::new(BidOrAsk::Ask, 5.0);
    let sell_order_from_sameep = Order::new(BidOrAsk::Bid, 22.5);

    engine.place_limit_order(pair.clone(), dec!(10.0), buy_order_from_ayush).unwrap();
    engine.place_limit_order(pair.clone(), dec!(10.5), buy_order_from_sameep).unwrap();
    engine.place_limit_order(pair.clone(), dec!(11.5), sell_order_from_ayush).unwrap();
    engine.place_limit_order(pair.clone(), dec!(12.0), sell_order_from_sameep).unwrap();

    //Show state of Orderbook
    engine.show_orderbook_state();

    // Demonstrate market order functionality
    let mut market_buy_order = &mut Order::new(BidOrAsk::Bid, 1.0);
    engine.place_market_order(pair.clone(), market_buy_order);

    let mut market_sell_order = &mut Order::new(BidOrAsk::Ask, 2.0);
    engine.place_market_order(pair.clone(), market_sell_order);

    //Show state of Orderbook
    engine.show_orderbook_state();

}
