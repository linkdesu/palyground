use sparse_merkle_tree::{blake2b::Blake2bHasher, SMTBuilder};
use sparse_merkle_tree::default_store::DefaultStore;
pub use sparse_merkle_tree::MerkleProof;
use sparse_merkle_tree::{SparseMerkleTree, H256};

pub type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

fn main() {
    let mut smt = SMT::default();
    let key_1 = H256::from([1u8; 32]);
    let value_1 = H256::from([1u8; 32]);
    let key_2 = H256::from([2u8; 32]);
    let value_2 = H256::from([2u8; 32]);
    let key_3 = H256::from([3u8; 32]);
    let value_3 = H256::from([3u8; 32]);

    smt.update_all(vec![
        (key_1, value_1),
        (key_2, value_2),
        (key_3, value_3),
    ]);

    let proof = smt.merkle_proof(vec![key_1, key_2, key_3]).unwrap();
    let compiled_proof = proof.compile(vec![
        key_1,
        key_2,
        key_3,
    ]).unwrap();
    println!("compiled_proof = 0x{}", hex::encode(&compiled_proof.0));

    let mut smt_c_builder = SMTBuilder::new();
    smt_c_builder = smt_c_builder.insert(&key_1, &value_1).unwrap();
    smt_c_builder = smt_c_builder.insert(&key_2, &value_2).unwrap();
    smt_c_builder = smt_c_builder.insert(&key_3, &value_3).unwrap();

    let smt_c = smt_c_builder.build().unwrap();
    let ret = smt_c.verify(smt.root(), &compiled_proof.0);

    assert!(ret.is_ok(), "Proof should be verified");
}
