# Simple Escrow Smart Contract (Rust)

A secure, state-based escrow agreement built in Rust.

## Contract Summary
This contract facilitates a trust-minimized transaction between a `Buyer` and a `Seller`. 
1. The **Buyer** deposits the agreed amount.
2. The funds are **Locked** inside the contract.
3. Once the Buyer receives the item/service, they **Confirm Delivery**.
4. The funds are released to the **Seller**.

If the Seller cannot fulfill the order, they can issue a **Refund** to return the funds to the Buyer.

## How to Build & Test

This project is a standard Rust library. You need `cargo` installed.

1. **Verify Installation**
   ```bash
   rustc --version
   cargo --version
   ```

2. **Run Tests**
   The logic is verified using Rust's built-in test harness.
   ```bash
   cd contract
   cargo test
   ```

   Expected output:
   ```text
   running 4 tests
   test tests::test_initialization ... ok
   test tests::test_deposit_flow ... ok
   test tests::test_release_flow ... ok
   test tests::test_refund_flow ... ok
   test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   ```

## State & Flows

The contract enforcement relies on the `EscrowState` enum:

1.  **AwaitingPayment**: Initial state. Waiting for `deposit()`.
2.  **AwaitingDelivery**: Funds are locked. Waiting for `confirm_delivery()` or `refund_buyer()`.
3.  **Completed**: Funds have been paid to the Seller. Terminal state.
4.  **Refunded**: Funds have been returned to the Buyer. Terminal state.

### Security Checks
- **Unauthorized**: Checks `sender == self.buyer` or `sender == self.seller` appropriate for each action.
- **InvalidState**: Prevents actions like "refunding" before money is deposited.

## Known Limitations
-   **No Expiry**: In a real implementation, we would need a time-lock (e.g., "Refund allowed after 30 days") so funds don't get stuck if the Buyer goes missing.
-   **In-Memory**: This is a pure logic implementation. To deploy to a chain (like Solana or Near), strict types (like `Pubkey` instead of `String`) and an entrypoint function would be added.

## Deployed Link
*Local Logic Simulation (Unit Tests)*
