# Coffee Token Component - Proof of Concept (PoC)

This repository contains the **Coffee Token Component**, a proof-of-concept (PoC) implementation that represents a token tied to the price of coffee in the USA region. The goal of this PoC is to demonstrate the core token functionality and its interactions with collateral.

## Overview

This version is focused solely on the token component itself and is intended for **testing purposes only**. For modularity, both the DAO and the oracle components are handled separately and will be integrated into future versions.

## Key Features

- **Price Management**: The token’s price is updated through a custom price updater mechanism, which will eventually be connected to an external oracle.
- **Buy and Sell Functionality**: Users can exchange collateral (XRD) for Coffee tokens using the buy and sell functions, with the buy function also applying a fee.
- **Shared Collateral**: A vault stores collateral as security for all minted Coffee tokens, ensuring stable backing.
- **Custom Badge-Based Authorization**: Badge-based access control is used for the price updater, though it is currently implemented in a simplified manner.

## Current Limitations

- **DAO and Oracle Not Yet Integrated**: This version does not include the decentralized governance (DAO) mechanism or the oracle for real-time price updates. These will be added in future iterations.
- **Naive Badge Implementation**: The current version uses badges in a basic way for access control, which will be refined in the production-ready implementation.

## Usage

The component allows users to:

1. **Instantiate** a Coffee token for the USA region, specifying collateral, initial price, and friction (buy/sell fee).
2. **Buy** Coffee tokens by providing collateral and paying a fee.
3. **Sell** Coffee tokens in exchange for collateral, minus a friction fee.

## Functions

- `get_price`: Returns the current price of the Coffee token in terms of collateral.
- `set_price`: Updates the token price. (Currently requires a badge, with more complex conditions to come.)
- `buy`: Exchanges collateral for Coffee tokens, minus a friction fee.
- `sell`: Exchanges Coffee tokens for collateral, minus a friction fee.

## Future Work

- **DAO Integration**: A decentralized autonomous organization (DAO) will be responsible for managing parameters that account for the specific characteristics of the market and optimizing behavior based on chosen objectives.
- **Oracle Integration**: The token’s price will be dynamically updated by an oracle in a future version.
