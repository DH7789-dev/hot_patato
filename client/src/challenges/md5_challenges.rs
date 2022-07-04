use std::collections::HashMap;
use std::ops::{ControlFlow};
use md5;
use common::challenges::IChallenge;
use common::model_md5_challenge::{MD5HashCashInput, MD5HashCashOutput};

pub struct MD5HashCash {
    pub input: MD5HashCashInput,
    pub output: MD5HashCashOutput,
}

impl IChallenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("MD5HashCash")
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCash {
            input,
            output: MD5HashCashOutput { seed: 0, hashcode: "".to_string()}
        }
    }

    fn solve(&self) -> Self::Output {
        let input = self.input.message.clone();
        let mut seed: u64 = 0;
        let mut output: MD5HashCashOutput;

        loop {

            let  exa_seed = format!("{:016X}", seed);
            let  hashcode = md5::compute(exa_seed + &input);
            let  str_md5 = format!("{:032X}", hashcode);

            output = MD5HashCashOutput {
                seed,
                hashcode: str_md5.clone()
            };

            if self.verify(MD5HashCashOutput {
                seed,
                hashcode: str_md5.clone()
            }) {
                println!("seed : {}", seed);
                break;
            }

            seed +=1;
        }
        return output;
    }

    fn verify(&self, output: Self::Output) -> bool {
        let decimal = u128::from_str_radix(&*output.hashcode, 16).unwrap();
        let binaire_size_all = format!("{:0128b}", decimal).len() as u32;
        let binaire_size = format!("{:b}", decimal).len() as u32;
        (binaire_size_all - binaire_size) >= self.input.complexity
    }
}