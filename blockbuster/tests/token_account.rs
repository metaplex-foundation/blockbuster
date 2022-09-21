use flatbuffers::FlatBufferBuilder;

async fn token_mint_account_success() {
    let mut builder = FlatBufferBuilder::new();

    //  let builder = FlatBufferBuilder::new();
    //     let builder = serialize_account(builder, &account, slot, is_startup);
    //     let owner = bs58::encode(account.owner).into_string();
    //     // Send account info over channel.
    //     runtime.spawn(async move {
    //         let data = SerializedData {
    //             stream: ACCOUNT_STREAM,
    //             builder,
    //         };
    //         let _ = sender.send(data).await;
    //     });
}
