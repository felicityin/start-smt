use std::vec;

use ckb_types::h160;
use ethereum_types::H160;
use sparse_merkle_tree::{
    blake2b::Blake2bHasher, default_store::DefaultStore, traits::Value, SparseMerkleTree, H256,
};

type Smt = SparseMerkleTree<Blake2bHasher, LeafValue, DefaultStore<LeafValue>>;

pub fn run() {
    let key = H160::from_slice(h160!("0x743a7e3b0b45fff5af4857d619e232fc9f86af1c").as_bytes());
    let value = 100u128;

    let kv = UserAmount {
        user:   key,
        amount: value,
    };

    let kvs = vec![kv].into_iter().collect::<Vec<UserAmount>>();

    let smt = construct_smt(&kvs);

    let root = *smt.root();
    println!("root: {:?}", root);

    let proof = smt
        .merkle_proof(vec![SmtKeyEncode::Address(key).to_h256()])
        .unwrap();
    println!("proof: {:?}", proof);

    let leaves = vec![(
        SmtKeyEncode::Address(key).to_h256(),
        SmtValueEncode::Amount(value).to_leaf_value().to_h256(),
    )];

    let ok = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();
    println!("ok: {}", ok);
}

fn construct_smt(kvs: &[UserAmount]) -> Smt {
    let kvs: Vec<(H256, LeafValue)> = kvs
        .iter()
        .map(|s| {
            (
                SmtKeyEncode::Address(s.user).to_h256(),
                SmtValueEncode::Amount(s.amount).to_leaf_value(),
            )
        })
        .collect();

    let mut smt = Smt::default();
    smt.update_all(kvs).expect("update");
    smt
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UserAmount {
    pub user:   H160,
    pub amount: u128,
}

pub enum SmtKeyEncode {
    Address(H160),
}

impl SmtKeyEncode {
    pub fn to_h256(&self) -> H256 {
        // Encode different types into SMT key type H256
        match self {
            SmtKeyEncode::Address(address) => {
                let mut buf = [0u8; 32];
                buf[..20].copy_from_slice(&address.to_fixed_bytes());
                buf.into()
            }
        }
    }
}

pub enum SmtValueEncode {
    Amount(u128),
}

impl SmtValueEncode {
    pub fn to_leaf_value(&self) -> LeafValue {
        // Encode different type to LeafValue
        match self {
            SmtValueEncode::Amount(amount) => (*amount).into(),
        }
    }
}

impl From<u128> for LeafValue {
    fn from(amount: u128) -> Self {
        let amount_bytes = amount.to_le_bytes();
        let mut buf = [0u8; 32];
        buf[..16].copy_from_slice(&amount_bytes);
        LeafValue(buf)
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct LeafValue(pub [u8; 32]);

impl Value for LeafValue {
    fn to_h256(&self) -> H256 {
        self.0.into()
    }

    fn zero() -> Self {
        Self([0u8; 32])
    }
}
