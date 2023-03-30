#![allow(dead_code)]

mod field_element;
mod utils;

#[cfg(test)]
mod tests {
    use crate::{field_element::FieldElement, utils::lambda};

    #[test]
    fn rsa_encryption() {
        // Key Generation.
        let p = 61;
        let q = 53;

        let n = p * q;
        let l_n = lambda(p, q);

        let e = FieldElement::new(17, l_n);
        let d = e.inv();

        let public_key = (e, n);
        let private_key = (d, n);

        // Encryption / Decryption.
        let message = FieldElement::new(1234, n);
        let ciphertext = message.pow(public_key.0);

        let message_prime = ciphertext.pow(private_key.0);

        assert_eq!(message, message_prime);
    }
}
