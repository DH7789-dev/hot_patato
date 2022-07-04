use common::challenges::IChallenge;
use common::model_recover_secret::{RecoverSecretInput, RecoverSecretOutput};

pub struct RecoverSecret {
    pub input: RecoverSecretInput,
    pub output: RecoverSecretOutput
}

impl IChallenge for RecoverSecret {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        String::from("RecoverSecret")
    }

    fn new(input: Self::Input) -> Self {
        RecoverSecret {
            input,
            output: RecoverSecretOutput { secret_sentence: "".to_string()}
        }
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: Self::Output) -> bool {
        println!("{:?}", answer);
        todo!()
    }
}