use std::vec;

use ckb_types::{h160, H160};
use sparse_merkle_tree::{
    blake2b::Blake2bHasher, default_store::DefaultStore, traits::Value, SparseMerkleTree, H256,
};

type Smt = SparseMerkleTree<Blake2bHasher, V, DefaultStore<V>>;

pub fn run() {
    let key = h160_to_arr(&h160!("0x743a7e3b0b45fff5af4857d619e232fc9f86af1c"));
    let value = 100u128;

    let kv = KV {
        addr:   key,
        amount: value,
    };

    let kvs = vec![kv].into_iter().collect::<Vec<KV>>();

    let smt = construct_smt(&kvs);

    let root = *smt.root();
    println!("root: {:?}", root);

    let proof = smt.merkle_proof(vec![arr_to_h256(&key)]).unwrap();
    println!("proof: {:?}", proof);

    let leaves = vec![(arr_to_h256(&key), V(value).to_h256())];

    let ok = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();
    println!("ok: {}", ok);
}

fn construct_smt(kvs: &[KV]) -> Smt {
    let kvs: Vec<(H256, V)> = kvs
        .iter()
        .map(|s| (arr_to_h256(&s.addr), V(s.amount)))
        .collect();

    let mut smt = Smt::default();
    smt.update_all(kvs).expect("update");
    smt
}

fn h160_to_arr(v: &H160) -> [u8; 20] {
    let mut arr = [0u8; 20];
    arr.copy_from_slice(v.as_bytes());
    arr
}

fn arr_to_h256(arr: &[u8; 20]) -> H256 {
    let mut buf = [0u8; 32];
    buf[..20].copy_from_slice(arr);
    buf.into()
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct KV {
    pub addr:   [u8; 20],
    pub amount: u128,
}

#[derive(Default, Clone, Copy, Debug)]
struct V(pub u128);

impl Value for V {
    fn to_h256(&self) -> H256 {
        let mut buf = [0u8; 32];
        let amount_bytes = self.0.to_le_bytes();
        buf[..16].copy_from_slice(&amount_bytes);
        buf.into()
    }
    fn zero() -> Self {
        Default::default()
    }
}
