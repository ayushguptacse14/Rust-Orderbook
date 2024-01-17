#![allow(dead_code)]

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(),
            BidOrAsk::Ask => self.bid_limits()
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
        
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a,b|a.price.cmp(&b.price));
        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a,b|b.price.cmp(&a.price));
        limits
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {

        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
    pub fn delete_limit_order(&mut self, price: Decimal, order_id: u64) -> bool {
        let is_deleted_from_bids = self.bids
            .entry(price)
            .and_modify(|limit| limit.orders.retain(|order| order.id != order_id))
            .or_insert_with(|| Limit::new(price))
            .orders
            .iter()
            .any(|order| order.id == order_id);

        let is_deleted_from_asks = self.asks
            .entry(price)
            .and_modify(|limit| limit.orders.retain(|order| order.id != order_id))
            .or_insert_with(|| Limit::new(price))
            .orders
            .iter()
            .any(|order| order.id == order_id);

        is_deleted_from_bids || is_deleted_from_asks
    }
}


#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    id: u64,
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(id: u64, bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { id, bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook= Orderbook::new();
        orderbook.add_limit_order(dec! (500), Order:: new(100, BidOrAsk::Ask, 10.0)); 
        orderbook.add_limit_order(dec! (200), Order:: new(101, BidOrAsk::Ask, 10.0)); 
        orderbook.add_limit_order(dec! (100), Order:: new(102, BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec! (300), Order:: new(103, BidOrAsk::Ask, 10.0));

        let mut market_order = Order::new(104,BidOrAsk::Bid, 10.0);
        orderbook. fill_market_order (&mut market_order);

        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get (0) .unwrap();
        assert_eq!(matched_limit.price, dec! (100));
        assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.orders.get(0) .unwrap();
        assert_eq!(matched_order.is_filled(), true);
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(105, BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(106, BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0)
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(109, BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(110, BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(111, BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(107, BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(108, BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}
