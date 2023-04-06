#![allow(dead_code)]

mod salsa20;
mod utils;

#[cfg(test)]
mod tests {
    use crate::salsa20::salsa20_encrypt;

    #[test]
    fn salsa20_encryption() {
        let key0 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key1 = [
            201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216,
        ];
        let nonce = [
            101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
        ];
        let message = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        ];

        let ciphertext = salsa20_encrypt(key0, key1, nonce, message);
        let message_prime = salsa20_encrypt(key0, key1, nonce, ciphertext);

        assert_eq!(message, message_prime);
    }
}
