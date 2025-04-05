
use std::io::stdin;
mod tic_tac_toe;
use tic_tac_toe::tic_tac_toe ;
mod hangman;
use hangman::hangman;
fn main(){
    println!("Welcome to Command-Line Games Complex written in Rust") ;
    loop {
        println!("select the following number");
        println!("play tic tac toe game -> 1") ;
        println!("play hangman game -> 2");
        println!("exit -> 0");
        let mut input = String::new() ;
        stdin().read_line(&mut input).expect("Error Occured while taking the input");
        input = input.trim().to_string();
        let value_check = input.to_string().into_bytes()[0] ;
        if value_check >= 48 && value_check <= 50 { // as we have only 3 options    
            let value:u8 = input.parse().expect("Error Occured While Parsing");
            if value == 0 {break}
            else if value == 1{ tic_tac_toe();}
            else{ hangman(); }
        }else{
            println!("Invalid Option Choosed!");
        }
    }
}