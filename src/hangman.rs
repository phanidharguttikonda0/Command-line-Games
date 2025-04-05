
use std::{thread::sleep, time::Duration};
use std::io::stdin;


#[derive(Debug)]
enum Grade {
	Excellent,
	OutStanding,
	VeryGood,
	Good,
	Average,
	BelowAverage
}


fn solve(mut present_actual_value: Vec<char>, entered_char: char, mut orginal_value: Vec<char>) -> (String,String, bool) {
	let mut i = 0 ;
	let mut sucess = false ;
	while i < present_actual_value.len() {
		if entered_char == orginal_value[i] && present_actual_value[i] == '_'{
			present_actual_value[i] = entered_char ;
			orginal_value[i] = '_';
			sucess = true ;
			break;
		}
		i += 1 ;
	}
	(present_actual_value.iter().collect(), orginal_value.iter().collect(), sucess)
}

pub fn hangman() {
	println!("Welcome to hangman Game") ;
	println!("Read the Following instructions of the game") ;
	println!("You will be having a total of 7 chances") ;
	println!("after 7 chances you will be lost");
	println!("the player-1 set's a letter and then suggests which joner it was"); 
	sleep(Duration::from_secs(10)); // we are stoping from taking the input for 10 seconds to
	// read the instructions carefully
	let mut total_questions:u8 = 0 ;
	let mut total_correct:u8 = 0 ;
	println!("let's starting giving the value");
	loop{
		println!("set the word name");
		let mut word_name = String::new() ;
		stdin().read_line(&mut word_name).expect("error while taking input word_name");
		let mut word_jouner = String::new() ;
		println!("set the word jouner");
		stdin().read_line(&mut word_jouner).expect("error while taking input word_jouner");
		word_name = word_name.trim().to_string() ;
		let mut current_word = word_name.clone() ;
		let mut user_gussed_word = String::new() ;
		if word_name.len() > 2 && word_jouner.len() > 2{
			total_questions += 1 ;
			for _ in 0..word_name.len() {
				user_gussed_word.push('_');
			}
			let mut lifes = 7;
			loop {
				println!("jouner was : {}", word_jouner);
				println!("Fill the word {} length of word was {}", user_gussed_word, user_gussed_word.len());
				println!("Enter a character ") ;
				let mut character = String::new() ;
				stdin().read_line(&mut character).expect("msg");
				character = character.trim().to_string();
				if character.len() > 1 {
					character = (&character[0..1]).to_string();
				}
				// now we are going to call the solve
				let charac:Vec<char> = character.chars().collect() ;
				let success:bool ;
				(user_gussed_word, current_word,success) = solve(user_gussed_word.chars().collect(),
				 	charac[0], 
				 	current_word.chars().collect()
					); // we will get new user_guessed_word and current_word
				if !success {
					lifes -= 1;
					if lifes == 0 {
						println!("No lives, try next time, the answer was {}", word_name) ;
						break;
					}
				}
				let check:Vec<char> = user_gussed_word.chars().collect();
				let mut completed = true;
				check.into_iter().for_each(|value| {
					if value == '_' {
						completed = false;
					}
				});
				println!("{}",success) ;
				if completed {
					println!("Hurray Answer was Correct");
					total_correct += 1;
					break;
				}
			}


		}else{
			println!("Word name and jouner name must have length 3 or more") ;
		}
		println!("if you want to exit the game click 0");
		let mut opinion = String::new() ;
		stdin().read_line(&mut opinion).expect("error while taking the input for opinion") ;
		opinion = opinion.trim().to_string();
		if opinion == "0"{
			break;
		}
	}
	println!("Total questions asked {}", total_questions);
	println!("total correct answers {}", total_correct) ;
	let percentage = (total_correct/total_questions) * 100 ;
	if percentage >= 90 {
		println!("{:#?}", Grade::OutStanding) ;
	}else if percentage >=80 {
		println!("{:#?}", Grade::Excellent) ;
	}else if percentage >= 70 {
		println!("{:#?}", Grade::VeryGood);
	}else if percentage >= 60 {
		println!("{:#?}", Grade::Good) ;
	}
	else if percentage >= 50 {
	    println!("{:#?}", Grade::Average);
	}else {
	    println!("{:#?}", Grade::BelowAverage);
	}
}


