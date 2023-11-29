## Rust Script Overview

This Rust script is an asynchronous application that periodically queries the Kraken API for information on the last trading price and lot volume of a specific trading pair. Here's a brief breakdown:

### Dependencies
- **serde::Deserialize**: Used for deserializing JSON responses from the Kraken API into Rust data structures.
- **error_chain::error_chain**: Facilitates error handling in a more ergonomic way.
- **std::collections::HashMap**: Used for storing JSON response data.
- **tokio::time::sleep** and **std::time::Duration**: Used for introducing delays in the program.

### `error_chain!` Macro
Generates code for more convenient error handling, creating error types for I/O, HTTP request, and JSON deserialization errors.

### Data Structures
- **KrakenTickerResponse**: Represents the structure of the Kraken API JSON response for ticker information.
- **PairData**: Represents specific data for a trading pair in the response.

### Functions
- **get_last_traded_closed_price_kraken** and **get_last_traded_closed_lot_volume_kraken**: Asynchronous functions that make requests to the Kraken API to retrieve the last trading price and lot volume, respectively.
- **run**: The main function runs an infinite loop that periodically queries the Kraken API, prints the results, and introduces a 10-second delay between each request.
- **main**: The main function calls `run` and prints error details in case of failure.
- **print_error_details**: A utility function that prints error details, displaying each cause separately.

### Attribute
- **#[tokio::main]**: Indicates that the `main` function uses the Tokio runtime to support asynchronous operations.

In summary, this Rust script is an asynchronous program designed to periodically fetch trading information from the Kraken API, with explicit error handling.
