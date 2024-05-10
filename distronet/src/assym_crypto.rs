
/// Generate nonce to be encrypted with user's public key
fn gen_nonce() -> u64 {
    234
}


mod tests {
    use super::*;

    #[test]
    fn test_gen_nonce() {
        assert_eq!(gen_nonce(), 234);
    }
}