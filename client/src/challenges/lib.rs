#[cfg(test)]
mod tests {
    use common::challenges::IChallenge;
    use common::model_md5_challenge::{MD5HashCashInput, MD5HashCashOutput};
    use common::model_recover_secret::{RecoverSecretInput, RecoverSecretOutput};
    use crate::{MD5HashCash, RecoverSecret};

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

    #[test]
    fn test_secret() {
        let secret_input: RecoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: vec![3, 4, 5, 7, 7, 3]
        };
        let recover_secret: RecoverSecret = RecoverSecret::new(secret_input);
        let recover_secret_output: RecoverSecretOutput = recover_secret.solve();
        assert_eq!(recover_secret_output.secret_sentence, "C'est chou".to_string());
    }
}