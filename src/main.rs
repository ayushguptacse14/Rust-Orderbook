mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order};
use rust_decimal_macros::dec;

fn main() {
    // Initialize the orderbook and matching engine
    let mut engine = MatchingEngine::new();

    // Create a trading pair
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

    // Add the market to the matching engine
    engine.add_new_market(pair.clone());

    //Show state of Orderbook
    engine.show_orderbook_state();

    // Create orders
    let buy_order_from_ayush = Order::new(1, BidOrAsk::Bid, 7.5);
    let buy_order_from_sameep = Order::new(2, BidOrAsk::Bid, 20.5);
    let sell_order_from_ayush = Order::new(3, BidOrAsk::Ask, 5.0);
    let sell_order_from_sameep = Order::new(4, BidOrAsk::Bid, 22.5);

    // Place limit orders with error handling
    let orders = [
        (dec!(10.0), buy_order_from_ayush),
        (dec!(10.5), buy_order_from_sameep),
        (dec!(11.5), sell_order_from_ayush),
        (dec!(12.0), sell_order_from_sameep),
    ];

    for (price, order) in orders.iter() {
        match engine.place_limit_order(pair.clone(), *price, order.clone()) {
            Ok(_) => println!("Order placed at price: {}", price),
            Err(e) => eprintln!("Error placing order at price {}: {}", price, e),
        }
    }

    //Show state of Orderbook
    engine.show_orderbook_state();

    // Delete a limit order with error handling
    match engine.delete_limit_order(pair.clone(), dec!(10.0), 1) {
        Ok(_) => println!("Order deleted successfully"),
        Err(e) => eprintln!("Error deleting order: {}", e),
    }

    //Show state of Orderbook
    engine.show_orderbook_state();

    // Get volume
    if let Err(e) = engine.get_volume(pair.clone(), dec!(12.0), BidOrAsk::Bid) {
        eprintln!("Error getting volume: {}", e);
    }

    // Demonstrate market order functionality with error handling
    let market_buy_order = &mut Order::new(5, BidOrAsk::Bid, 1.0);
    if let Err(e) = engine.place_market_order(pair.clone(), market_buy_order) {
        eprintln!("Error placing market buy order: {}", e);
    }

    let market_sell_order = &mut Order::new(6, BidOrAsk::Ask, 2.0);
    if let Err(e) = engine.place_market_order(pair.clone(), market_sell_order) {
        eprintln!("Error placing market sell order: {}", e);
    }

    //Show state of Orderbook
    engine.show_orderbook_state();
}
