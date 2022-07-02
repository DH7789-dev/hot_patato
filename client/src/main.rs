use std::ptr::null;
use common::challenges::IChallenge;
use rand::Rng;
use common::md5challenge::{MD5HashCash, MD5HashCashOutput};
use common::model::{Subscribe, PublicPlayer, Challenge, ChallengeAnswer, ChallengeResult, JsonMessage};
use crate::tcp_client::TcpClient;

mod tcp_client;

fn main() {

    let mut round: u32 =0;
    let player = "billy".to_string();
    let mut tcp_client = TcpClient::initialize();

    println!(" -- Hello :");
    let hello = JsonMessage::Hello;
    let json_message: JsonMessage = tcp_client.send(&hello);
    println!("{:?}", json_message);

    println!(" -- Subscribe :");
    let name = String::from(player.clone());
    let subscribe = JsonMessage::Subscribe(Subscribe {name});
    let subscribe_response: JsonMessage = tcp_client.send(&subscribe);
    println!("{:?}", subscribe_response);

    println!(" -- waiting PlayerBoard :");
    let player_board: JsonMessage = tcp_client.waiting_response();
    let players: &Vec<PublicPlayer>;

    match &player_board {
        JsonMessage::PublicLeaderBoard(board) => {
            players = &*board
        } _ => {
            println!("Houston, le probleme est pas ici");
            return;
        }
    }

    println!("{:?}", players);


    println!(" -- Challenge :");
    let mut random = rand::thread_rng();

    loop {
        let server_response = tcp_client.waiting_response();

        match server_response {
            JsonMessage::Challenge(challenge) => {
                let mut target_index = random.gen_range(0..players.len());
                let mut target: &PublicPlayer = players.get(target_index).unwrap();
                while target.name == player {
                    target_index = random.gen_range(0..players.len());
                    target = players.get(target_index).unwrap();
                }

                match challenge {

                    //MD5Challenge
                    Challenge::MD5HashCash(input) => {
                        round += 1;
                        println!("Start of ROUND : {}", round );
                        println!("{:?}", input );
                        let md5_challenge: MD5HashCash = MD5HashCash::new(input);
                        let md5_output: MD5HashCashOutput = md5_challenge.solve();
                        let response: JsonMessage = JsonMessage::ChallengeResult(ChallengeResult {
                            next_target: target.name.clone(),
                            answer: ChallengeAnswer::MD5HashCash(md5_output)
                        });
                        tcp_client.send_message(
                            &response
                        );
                        println!(" Challenge Resolve");
                        println!("{:?}", response);
                    }
                }
            }
            JsonMessage::RoundSummary(round_summary) => {
                println!("{:?}", round_summary.chain);
            }
            JsonMessage::PublicLeaderBoard(board) => {
                println!("board")
            }
            JsonMessage::EndOfGame(_) => {
                println!("{:?}", server_response);
                break;
            }
            _ => {
                println!("there is a probleme")
            }
        }
    }


}


