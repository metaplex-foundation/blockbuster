use borsh::de::BorshDeserialize;
use mpl_bubblegum::{
    get_instruction_type,
    types::{LeafSchema, MetadataArgs},
    LeafSchemaEvent,
};
use spl_account_compression::events::AccountCompressionEvent;

fn hex_decode(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

/// Verifies that MintV1 instruction data with trailing bytes deserializes
/// correctly via lenient deserialization. Some client SDKs append extra
/// bytes that the on-chain program ignores; our parser must handle them too.
#[test]
fn mint_v1_with_trailing_bytes() {
    // Real MintV1 instruction data with 3 trailing bytes after MetadataArgs
    let outer_ix = hex_decode("9162c076b89376680d000000446f6d696e61746f72204e465403000000444f4d3e00000068747470733a2f2f78766f36652d76696161612d616161616e2d71343777612d6361692e696370302e696f2f646f6d696e61746f722d312d68712e706e67c800000100010000000000000000000000");

    let ix_type = get_instruction_type(&outer_ix);
    assert!(matches!(ix_type, mpl_bubblegum::InstructionName::MintV1));

    let ix_data = &outer_ix[8..];

    // Strict deserialization rejects trailing bytes
    assert!(MetadataArgs::try_from_slice(ix_data).is_err());

    // Lenient deserialization succeeds (matches on-chain behavior)
    let mut slice = ix_data;
    let args = MetadataArgs::deserialize(&mut slice).unwrap();
    let trailing = slice.len();

    assert_eq!(args.name, "Dominator NFT");
    assert_eq!(args.symbol, "DOM");
    assert_eq!(trailing, 3);
}

#[test]
fn leaf_schema_event_parses() {
    let noop_leaf = hex_decode("0100cb000000010000ff629531241899ac90cd494e9ced937314621e66c5f82156be7307a7467ab1d7f0675f00543ef5785fa8bab5cf78fee86b84a110604aab3247de3a179727fcbff0675f00543ef5785fa8bab5cf78fee86b84a110604aab3247de3a179727fcbf010000000000000086ac3fa3c1efc84138c16c05e26a119b75caf35cc1e1496aee8fe80670a88f94c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47001d6f9b936c3d2ab9d93154c77387a2c4af8dcce9802b303dee55d0647c9fd0a");

    let event = AccountCompressionEvent::try_from_slice(&noop_leaf).unwrap();
    match event {
        AccountCompressionEvent::ApplicationData(app_data) => {
            let spl_account_compression::events::ApplicationDataEvent::V1(v1) = app_data;
            let lse = LeafSchemaEvent::try_from_slice(&v1.application_data).unwrap();
            assert!(matches!(&lse.schema, LeafSchema::V1 { .. }));
        }
        _ => panic!("Expected ApplicationData"),
    }
}
