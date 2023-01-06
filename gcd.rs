fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(17, 35), 1);
    assert_eq!(gcd(24, 8), 8);
}
