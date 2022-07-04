#[cfg(test)]
mod tests {
    use common::challenges::IChallenge;
    use common::model_md5_challenge::{MD5HashCashInput, MD5HashCashOutput};
    use crate::MD5HashCash;

    #[test]
    fn test_md5() {
        let md5_input: MD5HashCashInput = MD5HashCashInput {
            message: "Je suis un message de test.".to_string(),
            complexity: 16,
        };
        let md5_challenge: MD5HashCash = MD5HashCash::new(md5_input);
        let md5_output: MD5HashCashOutput = md5_challenge.solve();
        assert_eq!(md5_output.hashcode, "0000C83DE6CCBA2EE1379A2E12AF97C9".to_string());
    }

}