// See: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub(crate) fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egcd_basic() {
        assert_eq!(egcd(3, 5), (1, 2, -1));
        assert_eq!(egcd(101, 13), (1, 4, -31));
        assert_eq!(egcd(123, 19), (1, -2, 13));
        assert_eq!(egcd(25, 36), (1, 13, -9));
        assert_eq!(egcd(69, 54), (3, -7, 9));
        assert_eq!(egcd(55, 79), (1, 23, -16));
        assert_eq!(egcd(33, 44), (11, -1, 1));
        assert_eq!(egcd(50, 70), (10, 3, -2));
    }
}
