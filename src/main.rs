use anyhow::Result;
use clap::Parser;
use sp_core::blake2_256;
use sp_core::crypto::{AccountId32, Ss58Codec};
use tracing::{Level, event};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// SS58-formatted address
    input_address: String,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing subscriber with appropriate log level
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set default tracing subscriber");

    let account_id = AccountId32::from_ss58check(&args.input_address)?;
    let evm_address = &AsRef::<[u8; 32]>::as_ref(&account_id)[0..20];
    let hashed_address: AccountId32 = {
        let mut data = [0u8; 24];
        data[0..4].copy_from_slice(b"evm:");
        data[4..24].copy_from_slice(evm_address);
        let hash = blake2_256(&data);

        AccountId32::new(hash)
    };
    let hashed_address_ss58 = hashed_address.to_ss58check();

    event!(Level::INFO, "AccountId (SS58): {}", &args.input_address);
    event!(Level::INFO, "AccountId: {}", hex::encode(&account_id));
    event!(Level::INFO, "---");
    event!(Level::INFO, "EVM address: {}", hex::encode(evm_address));
    event!(
        Level::INFO,
        "AccountId (hashed):  {}",
        hex::encode(hashed_address)
    );
    event!(Level::INFO, "AccountId (SS58): {}", hashed_address_ss58);

    Ok(())
}
