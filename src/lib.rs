use jni::JNIEnv;

use jni::objects::JClass;

use jni::sys::jstring;

use bdk::Wallet;
use bdk::database::MemoryDatabase;
use bdk::blockchain::{noop_progress, ElectrumBlockchain};
use bdk::electrum_client::Client;

use futures::executor;

async fn sync_balance_descriptor() -> Result<String, bdk::Error> {
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
        bitcoin::Network::Testnet,
        MemoryDatabase::default(),
        ElectrumBlockchain::from(client)
    )?;

    wallet.sync(noop_progress(), None)?;
    return Ok(wallet.get_balance()?.to_string());
}

#[no_mangle]
pub extern "system" fn Java_SyncBalanceDescriptor_balance(env:JNIEnv,
                                                          _class: JClass)
                                                          -> jstring {
    let balance = executor::block_on(sync_balance_descriptor()).unwrap();

    let output = env.new_string(format!("Descriptor balance: {} SAT", balance))
        .expect("Couldn't create java string!");

    return output.into_inner();
}
