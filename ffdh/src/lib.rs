#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod ffdh;
mod group;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{ffdh::FFDH, group::ID};

    #[test]
    fn ffdh_basic() {
        let alice_ffdh = FFDH::new(ID::Five);
        let alice_public_key = alice_ffdh.get_public_key();

        let bob_ffdh = FFDH::new(ID::Five);
        let bob_public_key = bob_ffdh.get_public_key();

        let alice_shared_secret = alice_ffdh.derive_shared_secret(&bob_public_key);
        let bob_shared_secret = bob_ffdh.derive_shared_secret(&alice_public_key);

        assert_eq!(alice_shared_secret, bob_shared_secret);
    }
}
