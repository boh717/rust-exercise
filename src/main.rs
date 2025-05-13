use rust_exercise::domain::input_record::InputRecord;
use rust_exercise::repository::client_repository::ClientRepositoryImpl;
use rust_exercise::repository::transaction_repository::TransactionRepositoryImpl;
use rust_exercise::service::transaction_service::TransactionService;
use std::env;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let filename = env::args().nth(1).ok_or_else(|| {
        anyhow::anyhow!("Missing filename argument. Usage: cargo run -- <filename.csv>")
    })?;

    let file = File::open(&filename)
        .map_err(|err| anyhow::anyhow!("Error opening file '{}': {}", filename, err))?;

    let client_repository = ClientRepositoryImpl::new();
    let transaction_repository = TransactionRepositoryImpl::new();
    let mut transaction_service =
        TransactionService::new(client_repository, transaction_repository);

    let records = InputRecord::from_csv(file);

    for r in records {
        match r {
            Ok(record) => {
                if let Err(err) = transaction_service.process_transaction(&record) {
                    eprintln!("Skipping transaction {}: {}", record.tx, err);
                }
            }
            Err(err) => {
                eprintln!("Error parsing record: {}", err);
            }
        }
    }

    println!("client,available,held,total,locked");
    for c in transaction_service.get_all_clients() {
        println!(
            "{},{},{},{},{}",
            c.client,
            c.available.get(),
            c.held.get(),
            c.total.get(),
            c.locked
        );
    }

    Ok(())
}
