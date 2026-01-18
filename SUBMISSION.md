# Build-And-Learn-Week Submission: Simple Escrow Contract

Here is a summary of the Simple Escrow contract designed for this challenge:

*   **Core Concept**: The contract acts as a neutral third party that holds funds securely until specific conditions are met, protecting both the buyer and the seller.
*   **State Machine**: The contract moves linearly through four distinct states: `AwaitingPayment` -> `AwaitingDelivery` -> `Completed` (or `Refunded`). This prevents operations from happening out of order (e.g., releasing funds before they are deposited).
*   **Role-Based Access Control**:
    *   **Buyer**: The only one allowed to `deposit` and `confirm_delivery`.
    *   **Seller**: The only one allowed to initiate a `refund` once funds are locked (preventing the buyer from "rug pulling" funds after goods are shipped).
*   **Safety First**: We use Rust's `Result` type for all operations to explicitly handle errors like `Unauthorized` access or `InvalidState` transitions. This ensures that the contract never crashes or enters an undefined state.
*   **Simulated Value Transfer**: The contract tracks balances internally (`u64`). In a real-world deployment (e.g., Solana/CosmWasm), these would correspond to actual token transfers, but the logic remains identical.
*   **Error Handling**: Custom `EscrowError` types provide clear feedback on why a transaction failed (e.g., "Insufficient funds", "Wrong user"), which is crucial for frontend integration.
*   **Testing**: The `lib.rs` includes a comprehensive unit test suite (`tests` module) that verifies the "happy path" (successful deposit & release) as well as security constraints (unauthorized refunds).
