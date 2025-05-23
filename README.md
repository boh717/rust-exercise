### Getting started

As requested, you can build the project with `cargo build` and run the program with:

```bash
cargo run -- <filename.csv>
```

Clients are printed to `stdout`, while errors are printed to `stderr`.

`cargo test` will run the unit tests.

### Assumptions

#### Operation constraints

The withdraw operation states the following:

> If a client does not have sufficient available funds the withdrawal should fail and the total amount of funds should not change

Dispute and resolve operations don't have the same constraints expressed. I made the following assumptions:

```rust
    pub fn dispute(&mut self, amount: f64) -> Result<(), ClientError> {
        if self.available.get() < amount {
            return Err(ClientError::FundsUpdateError {
                id: self.id.clone(),
                tx_type: "dispute".to_string(),
            });
        }
        ...
    }

    pub fn resolve(&mut self, amount: f64) -> Result<(), ClientError> {
        if self.held.get() < amount {
            return Err(ClientError::FundsUpdateError {
                id: self.id.clone(),
                tx_type: "resolve".to_string(),
            });
        }
        ...
    }
```

For disputes, I chose to prevent negative balances by checking available funds. This ensures clients can't have negative available balances.

For resolve operations, the check ensures we only release funds that are being held in dispute.

Chargebacks require no additional validation check because they operate on already disputed and held amounts, and apply only to valid disputed transactions.

#### Creation of new clients

The PDF states:

> There are multiple clients. Transactions reference clients. If a client doesn't exist create a new record

This works well for deposits, but for all other operations, the client should exist. For example, if a withdrawal is requested for a client that doesn't exist (the CSV might be malformed, or the client was deleted), I would create and output an empty client at the end (with available, total, and held set to 0). It doesn't look meaningful to have it.

### Design choices

To ensure only valid inputs, I deserialize the input data into the domain types directly (see `input_record.rs`). Every CSV field is validated before being turned into a valid domain type. Existing validations are trivial as input is controlled, but the current design, as requirements evolve, allows for adjustments to validations as needed, without much refactoring. This is a topic where Rust's type system helps a lot. Validating the input is also required for security reasons.

Another important choice was to use traits for the repositories. This allows my future self to swap the concrete implementation easily. At the moment, the repositories rely on a simple HashMap to store the data, but in the future when my system grows, I can decide to use a proper database.

The service has trait bounds on existing repositories. _In theory_, I could change my repositories without even touching the service (in practice, if I switch to a DB-based implementation, I would like to rely on transactions to create/update clients and transactions, so some refactoring would be needed).

On a more general note, the approach used is a hexagonal architecture, in which each layer has a well-defined responsibility. This also allows me to test each layer in isolation.

### Tests

I added unit tests for the most important parts of the code. I wanted to ensure that input was read correctly, client funds were updated as expected, and repositories/service didn't have any known issue (I hope!).

The service coverage is partial. I used some stubs for the important happy paths, but ignored some error paths. Given the scope of the exercise, it was a conscious choice to keep the code short.

### Performance

CSV is read through an iterator, so it should be performant even with large files. The output is also generated through an iterator (if you have a real database, you don't want to fetch all its data in memory).

The records are stored in a `HashMap` for simplicity. This defeats the use of iterators a bit as we will load most of the input records in memory, but it's a trade-off I made to keep the code simple.
