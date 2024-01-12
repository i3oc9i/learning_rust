use lazy_static::lazy_static;
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug)]
struct ExpirationDate {
	month: u32,
	year: u32,
}

#[derive(Debug)]
struct Card {
	number: u32,
	exp: ExpirationDate,
	cvv: u32,
}

type CreditCardDB = HashMap<&'static str, &'static str>;

lazy_static! {
	static ref CREDIT_CARD_DB: CreditCardDB = {
		let mut db = CreditCardDB::new();
		db.insert("Tim", "4111111 04 25 123");
		db.insert("Amy", "5061234 06 27 118");
		db.insert("Bob", "5856837 11 23 545");
		db.insert("Max", "6194941 12 23 117");
		db.insert("Eli", "3515683 11 24 119");
		db.insert("Bea", "3115463 01 23 934");
		db.insert("Ben", "37118401 07 24 jjj");
		db
	};
}

fn main() {
	println!("Enter name: ");

	let mut name = String::new();

	std::io::stdin()
		.read_line(&mut name)
		.expect("Failed to read line");

	let result = get_credit_card_info(&CREDIT_CARD_DB, name.trim());

	match result {
		Ok(card) => println!("Credit card info: {card:?}"),
		Err(e) => println!("Error: {e}"),
	}
}

fn get_credit_card_info(
	credit_card_db: &CreditCardDB,
	name: &str,
) -> Result<Card, String> {
	let card_string = credit_card_db
		.get(name)
		.ok_or(format!("No credit card found for name: {name}."))?;

	let card = parse_card(card_string)?;

	Ok(card)
}

fn parse_card(card: &str) -> Result<Card, String> {
	let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

	let len = numbers.len();
	let expected_len = 4;

	if len != expected_len {
		return Err(format!(
			"Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
		));
	}

	// We can safely unwrap here because we know the length of the vector is 4.
	let cvv = numbers.pop().unwrap();
	let year = numbers.pop().unwrap();
	let month = numbers.pop().unwrap();
	let number = numbers.pop().unwrap();

	Ok(Card {
		number,
		exp: ExpirationDate { month, year },
		cvv,
	})
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
	let numbers = card
		.split(' ')
		.into_iter()
		.map(|s| s.parse::<u32>())
		.collect::<Result<Vec<u32>, _>>()?;

	Ok(numbers)
}
