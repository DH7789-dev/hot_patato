use common::challenges::IChallenge;
use common::model_monstrous_maze::{MonstrousMazeInput, MonstrousMazeOutput};

pub struct MonstrousMaze {
    pub input: MonstrousMazeInput,
    pub output: MonstrousMazeOutput
}

impl IChallenge for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        String::from("MonstrousMaze")
    }

    fn new(input: Self::Input) -> Self {
        MonstrousMaze {
            input,
            output: MonstrousMazeOutput { path: "".to_string()}
        }
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}