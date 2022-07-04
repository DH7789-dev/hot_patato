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
        let  str_seed = format!("{:016X}", seed);
        let  hashcode = md5::compute(str_seed + &input);
        let  str_md5 = format!("{:032X}", hashcode);

        loop {

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
        let mut value = 0;
        let map_md5: MD5HashMap = MD5HashMap::initialize();
        let str_md5 = output.hashcode.clone();
        let string: Vec<char> = str_md5.chars().collect();
        println!("{:?}", str_md5);
        string
            .iter()
            .try_for_each(|exa| {
                let mapped_value: u32 = map_md5.get(exa.to_string());
                value += mapped_value;

                if mapped_value < 4 {
                    return ControlFlow::Break(exa);
                }
                ControlFlow::Continue(())
            });
        print!("{:?}", value);
        println!("  {:?}", self.input.complexity);
        return value >= self.input.complexity;
    }
}

pub struct MD5HashMap {
    map: HashMap<String, u32>,
}

impl MD5HashMap {
    pub fn initialize() -> MD5HashMap {
        let map: HashMap<String, u32> = HashMap::from([
            (String::from("0"), 4),
            (String::from("1"), 3),
            (String::from("2"), 2),
            (String::from("3"), 2),
            (String::from("4"), 1),
            (String::from("5"), 1),
            (String::from("6"), 1),
            (String::from("7"), 1),
            (String::from("8"), 0),
            (String::from("9"), 0),
            (String::from("A"), 0),
            (String::from("B"), 0),
            (String::from("C"), 0),
            (String::from("D"), 0),
            (String::from("E"), 0),
            (String::from("F"), 0),
        ]);
        MD5HashMap {
            map
        }
    }

    pub fn get(&self, key: String) -> u32 {
        *self.map.get(&key).unwrap()
    }
}