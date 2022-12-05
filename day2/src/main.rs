#![allow(non_snake_case)]
use std::{fs, fmt::Display};

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameToken {
	Rock,
	Paper,
	Scissors,
}

#[derive(Debug, PartialEq)]
enum MatchResult {
	Win,
	Loss,
	Draw,
	None,
}
#[derive(Debug, PartialEq)]
struct Playbook {
	 character: char,
	 token: GameToken,
}

struct Match {
	playerA: GameToken,
	playerB: GameToken,
}

impl Match {
	fn scoreForPlayerA(&self) -> u8 {
		0
	}

	// 6 for win, 3 for draw, 0 for loss
	fn playerAResult(&self) -> MatchResult {
		if self.playerA == GameToken::Rock {
			return match self.playerB {
				GameToken::Rock => MatchResult::Draw,
				GameToken::Paper => MatchResult::Loss,
				GameToken::Scissors => MatchResult::Win,
			};
		}
		if self.playerA == GameToken::Paper {
			return match self.playerB {
				GameToken::Rock => MatchResult::Win,
				GameToken::Paper => MatchResult::Draw,
				GameToken::Scissors => MatchResult::Loss,
			};
		}
		if self.playerA == GameToken::Scissors {
			return match self.playerB {
				GameToken::Rock => MatchResult::Loss,
				GameToken::Paper => MatchResult::Win,
				GameToken::Scissors => MatchResult::Draw,
			};
		}
		MatchResult::None
	}
	fn playerBScore(&self) -> u32 {
		let mut score = 0;
		score += match self.playerB {
			GameToken::Rock     => 1,
			GameToken::Paper    => 2,
			GameToken::Scissors => 3,
		};
		// note we're using the other player here
		score += match self.playerAResult() {
			MatchResult::Draw => 3,
			MatchResult::Loss => 6,
			MatchResult::Win  => 0,
			MatchResult::None => 0,
		};
		score
	}
}
// impl Display for GameToken {
// }
fn main() {
	let rules = vec![
		Playbook {
			character: 'A',
			token: GameToken::Rock,
		},
		Playbook {
			character: 'B',
			token: GameToken::Paper,
		},
		Playbook {
			character: 'C',
			token: GameToken::Scissors,
		},
		Playbook {
			character: 'X',
			token: GameToken::Rock,
		},
		Playbook {
			character: 'Y',
			token: GameToken::Paper,
		},
		Playbook {
			character: 'Z',
			token: GameToken::Scissors,
		},
	];
	dbg!(GameToken::Rock == GameToken::Scissors);

	let input_part_1 = fs::read_to_string("./fixtures/input_part_1.txt")
		.expect("could not parse input_part_1.txt from fixtures/ dir.");

    let matches = parseMactches(&input_part_1, &rules);
	let mut player_b_score: u32 = 0;
	for mach in matches {
		let result = mach.playerAResult();
		dbg!(" match: ", mach.playerA, mach.playerB);
		dbg!(result);
		player_b_score += mach.playerBScore();
	}
	println!("Player B (you)'s final score: {}", player_b_score);
}

// take a list of newline separated ABC space XYZ
// values and construct matches with them.
fn parseMactches(input: &str, rules: &Vec<Playbook>) -> Vec<Match> {

    let mut matches: Vec<Match> = Vec::new();

    let v: Vec<&str> =  input.split('\n').collect::<Vec<_>>();
	for letters in v {
		if letters.len() == 0 {
			break
		}
		let mut splitChar = letters.split(' ');
		let playerAChoice = splitChar.next().expect("Input matches format not correct").chars().next().expect("Input matches format not correct");
		let playerBChoice = splitChar.next().expect("Input matches format not correct").chars().next().expect("Input matches format not correct");
		let tokenA:Vec<&Playbook> = rules.into_iter().filter(|p| -> bool { p.character == playerAChoice }).collect();
		let tokenb:Vec<&Playbook> = rules.into_iter().filter(|p| -> bool { p.character == playerBChoice }).collect();
		// dbg!(&tokenA.first().unwrap().token, &tokenb.first().unwrap().token);
		matches.push(
			//Match {playerA: GameToken::Rock, playerB: GameToken::Scissors}
			Match {playerA: tokenA.first().unwrap().token, playerB: tokenb.first().unwrap().token}
		);
	}

    matches
}
