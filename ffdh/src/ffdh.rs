use crate::group::{Group, GROUPS, ID};
use num_bigint::{BigUint, RandBigInt, ToBigUint};

type PublicKey = BigUint;
type PrivateKey = BigUint;
type Secret = BigUint;

struct KeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

#[allow(clippy::upper_case_acronyms)]
pub(crate) struct FFDH<'a> {
    group: &'a Group,
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl FFDH<'_> {
    pub(crate) fn new(id: ID) -> Self {
        let group = GROUPS.get(&id).unwrap();
        let key_pair = FFDH::generate_keys(group);

        FFDH {
            group,
            private_key: key_pair.private_key,
            public_key: key_pair.public_key,
        }
    }

    fn generate_keys(group: &Group) -> KeyPair {
        let mut rng = rand::thread_rng();

        let min = 1.to_biguint().unwrap();
        let max = group.prime.clone() - 1.to_biguint().unwrap();

        let private_key = rng.gen_biguint_range(&min, &max);
        let public_key = group.generator.modpow(&private_key, &group.prime);

        KeyPair {
            private_key,
            public_key,
        }
    }

    pub(crate) fn get_public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub(crate) fn derive_shared_secret(&self, public_key: &PublicKey) -> Secret {
        public_key.modpow(&self.private_key, &self.group.prime)
    }
}
