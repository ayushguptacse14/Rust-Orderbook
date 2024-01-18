# Rust-based Cryptocurrency Matching Engine

## Introduction
This Rust project implements a cryptocurrency trading matching engine. It features a matching engine for trade orders, an order book to manage bids and asks, and supports trading pairs like BTC/USD. This system is designed for financial trading platforms that require efficient and accurate order matching.

## Installation and Setup
To install and set up this project, ensure you have Rust installed on your system. Clone the repository and install the required dependencies, including `rust_decimal`.

```bash
# Clone the repository
git clone https://github.com/ayushguptacse14/Rust-Orderbook.git

# Navigate to the project directory
cd Rust-Orderbook

# Build the project
cargo build
```

## Usage
To use the application, start by initializing the matching engine and order book. Then, create a trading pair, such as BTC to USD, and process orders through the engine.

```bash
# Run the project
cargo run
```

```rust
// Initialize the matching engine
let mut engine = MatchingEngine::new();

// Create a trading pair
let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

// Place a limit order using the matching engine
let buy_order_from_ayush = Order::new(1, BidOrAsk::Bid, 7.5);
let sell_order_from_ayush = Order::new(3, BidOrAsk::Ask, 5.0);

engine.place_limit_order(pair.clone(), dec!(10.0), buy_order_from_ayush).unwrap();
engine.place_limit_order(pair.clone(), dec!(11.5), sell_order_from_ayush).unwrap();

// Delete a limit order using the matching engine
engine.delete_limit_order(pair.clone(), dec!(10.0), 1).unwrap();

// Market order functionality
let mut market_buy_order = &mut Order::new(5, BidOrAsk::Bid, 1.0);
engine.place_market_order(pair.clone(), market_buy_order);

let mut market_sell_order = &mut Order::new(6, BidOrAsk::Ask, 2.0);
engine.place_market_order(pair.clone(), market_sell_order);
```

## Code Structure
- `engine.rs`: Defines the `MatchingEngine` and `TradingPair` structures. The `MatchingEngine` handles the logic for matching buy and sell orders, while `TradingPair` represents a pair of currencies in a trade.
- `orderbook.rs`: Contains the `Orderbook` structure and `BidOrAsk` enums. The `Orderbook` manages lists of bids and asks, and includes methods for order processing.
- `main.rs`: The main entry point of the application. It demonstrates initializing the matching engine, setting up an order book, and creating a BTC/USD trading pair.
