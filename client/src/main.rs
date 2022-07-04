use common::challenges::IChallenge;
use rand::Rng;
use common::model::{Subscribe, PublicPlayer, Challenge, ChallengeAnswer, ChallengeResult, JsonMessage};
use common::model_md5_challenge::MD5HashCashOutput;
use common::model_monstrous_maze::MonstrousMazeOutput;
use common::model_recover_secret::RecoverSecretOutput;
use crate::challenges::md5_challenges::MD5HashCash;
use crate::challenges::monstrous_maze::MonstrousMaze;
use crate::challenges::recover_secret::RecoverSecret;
use crate::tcp_client::TcpClient;

mod challenges;
mod tcp_client;

fn main() {

    let mut round: u32 =0;
    let player = "billy".to_string();
    let mut tcp_client = TcpClient::initialize();
    let mut leader_board: Vec<PublicPlayer> = vec!();

    println!(" -- Hello :");
    let hello = &JsonMessage::Hello;
    tcp_client.send_message(hello);


    loop {
        let server_response = tcp_client.waiting_response();
        println!("{:?}", server_response);
        match server_response {
            JsonMessage::Welcome(..) => {
                println!(" -- Subscribe :");
                let name = String::from(player.clone());
                let subscribe = JsonMessage::Subscribe(Subscribe {name});
                let subscribe_response: JsonMessage = tcp_client.send(&subscribe);
                println!("{:?}", subscribe_response);
            }
            JsonMessage::SubscribeResult(..) => {
                println!("SubscribeResult")
            }
            JsonMessage::RoundSummary(round_summary) => {
                println!("ROUND ");
                println!("{:?}", round_summary.chain);
            }
            JsonMessage::PublicLeaderBoard(board) => {
                leader_board = board;

                println!("{:?}", leader_board);

            }
            JsonMessage::EndOfGame(_) => {
                println!("{:?}", server_response);
                break;
            }
            JsonMessage::Challenge(challenge) => {
                println!(" -- Challenge :");
                let target = choose_next_target(&leader_board, &player);

                match challenge {

                    //MD5Challenge
                    Challenge::MD5HashCash(input) => {
                        round += 1;
                        println!("Start of ROUND : {}", round );
                        println!("{:?}", input );
                        let md5_challenge: MD5HashCash = MD5HashCash::new(input);
                        let md5_output: MD5HashCashOutput = md5_challenge.solve();
                        let challenge_result: JsonMessage = JsonMessage::ChallengeResult(ChallengeResult {
                            next_target: target.name.clone(),
                            answer: ChallengeAnswer::MD5HashCash(md5_output)
                        });
                        tcp_client.send_message(
                            &challenge_result
                        );
                        println!(" Challenge Resolve");
                        println!("{:?}", challenge_result);
                    },
                    //MonstrousMaze
                    Challenge::MonstrousMaze(input) => {
                        round += 1;
                        println!("Start of ROUND : {}", round );
                        println!("{:?}", input );
                        let mm_challenge: MonstrousMaze = MonstrousMaze::new(input);
                        let mm_output: MonstrousMazeOutput = mm_challenge.solve();
                        let challenge_result: JsonMessage = JsonMessage::ChallengeResult(ChallengeResult {
                            next_target: target.name.clone(),
                            answer: ChallengeAnswer::MonstrousMaze(mm_output)
                        });
                        tcp_client.send_message(
                            &challenge_result
                        );
                        println!(" Challenge Resolve");
                        println!("{:?}", challenge_result);
                    },
                    //RecoverSecret
                    Challenge::RecoverSecret(input) => {
                        round += 1;
                        println!("Start of ROUND : {}", round );
                        println!("{:?}", input );
                        let rs_challenge: RecoverSecret = RecoverSecret::new(input);
                        let rs_output: RecoverSecretOutput = rs_challenge.solve();
                        let challenge_result: JsonMessage = JsonMessage::ChallengeResult(ChallengeResult {
                            next_target: target.name.clone(),
                            answer: ChallengeAnswer::RecoverSecret(rs_output)
                        });
                        tcp_client.send_message(
                            &challenge_result
                        );
                        println!(" Challenge Resolve");
                        println!("{:?}", challenge_result);
                    },
                }
            }
            _ => {
                println!("there is a probleme")
            }
        }
    }


}

fn choose_next_target( leader_board: &Vec<PublicPlayer>, player: &String) -> PublicPlayer {
    let mut random = rand::thread_rng();
    let mut target_index = random.gen_range(0..leader_board.len());
    let mut target: &PublicPlayer = leader_board.get(target_index).unwrap();
    while &target.name == player {
        target_index = random.gen_range(0..leader_board.len());
        target = leader_board.get(target_index).unwrap();
    }
    target.clone()
}

