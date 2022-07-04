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
        // let mut letters: Vec<char> = self.input.letters.chars().collect();
        // let tuple = self.input.tuple_sizes.clone();
        // let word_count = self.input.word_count.clone();
        // let mut secret_string: String = String::new();
        // let mut cursor: usize = 0;
        //
        // for i in tuple {
        //     for j in cursor..(i + cursor) {
        //         if !is_exist(&secret_string, letters[j] ) {
        //             secret_string.push(letters[j])
        //         }
        //     }
        //
        //     cursor += i
        // }
        //
        // println!("{:?}", secret_string);
        RecoverSecretOutput {
            secret_sentence: String::new()
        }
    }

    fn verify(&self, answer: Self::Output) -> bool {
        println!("{:?}", answer);
        todo!()
    }

}

// fn is_exist(string_output: &String, letter: char) -> bool {
//     let values: Vec<char> = string_output.chars().collect();
//     for i in values {
//         if i == letter {
//             return true;
//         }
//     }
//     return false;
// }
//
// fn reorganised(string_output: &String, tuple_str: String, letters: String) -> String {
//     // let values: Vec<char> = string_output.chars().collect();
//     let tuple: Vec<char> = tuple_str.chars().collect();
//
//     for i in 0..tuple.len() {
//         if i > 0 {
//             if !is_exist(string_output, tuple[i]) {
//                 println!("je sais pas")
//             }
//
//         }
//     }
//     return String::new()
// }