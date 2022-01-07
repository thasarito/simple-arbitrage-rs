# Reimplement Simple Arbitrage in Rust

## Example

- [simple-arbitrage](https://github.com/flashbots/simple-arbitrage)
- [yield-liquidator](https://github.com/yieldprotocol/yield-liquidator)

## Stack

- [foundry](https://github.com/gakonst/foundry)
- [ethers.rs](https://github.com/gakonst/ethers-rs)

## Arbitrage Logic

1. Find TKN/ETH pairs that exist in several Dexes.

2. Update those pairs reserve using `getReserve` from `UniswapFlashQuery.sol`

3. store reserve in `Reserve` struct `WETH` should always be `reserve1`

4. calculated `buy_price` and `sell_price`
