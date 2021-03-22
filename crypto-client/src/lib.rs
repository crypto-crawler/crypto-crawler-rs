/// An unified restful client for all cryptocurrency exchanges.
pub struct CryptoClient {
    #[allow(dead_code)]
    exchange: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
