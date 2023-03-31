#[path = "../src/calculator/mod.rs"]
mod calculator;
use calculator::calculate;

#[test]
fn test_01_numerical_literal() {
	assert_eq!(
		calculate(String::from("0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("4"), &mut calculator::environment::new()).unwrap(),
		4.0
	);

	assert_eq!(
		calculate(String::from("0.0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("4.5"), &mut calculator::environment::new()).unwrap(),
		4.5
	);
	assert_eq!(
		calculate(String::from("455.555"), &mut calculator::environment::new()).unwrap(),
		455.555
	);
}

#[test]
fn test_02_sign() {
	assert_eq!(
		calculate(String::from("-0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("-4"), &mut calculator::environment::new()).unwrap(),
		-4.0
	);
	assert_eq!(
		calculate(String::from("-0.0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("-4.5"), &mut calculator::environment::new()).unwrap(),
		-4.5
	);
	assert_eq!(
		calculate(
			String::from("-455.555"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		-455.555
	);

	assert_eq!(
		calculate(String::from("+0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("+4"), &mut calculator::environment::new()).unwrap(),
		4.0
	);
	assert_eq!(
		calculate(String::from("+4.5"), &mut calculator::environment::new()).unwrap(),
		4.5
	);

	assert_eq!(
		calculate(String::from("--4"), &mut calculator::environment::new()).unwrap(),
		4.0
	);
	assert_eq!(
		calculate(String::from("+-4"), &mut calculator::environment::new()).unwrap(),
		-4.0
	);
}

#[test]
fn test_03_additive() {
	assert_eq!(
		calculate(String::from("0 + 0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("4 + 3"), &mut calculator::environment::new()).unwrap(),
		7.0
	);
	assert_eq!(
		calculate(String::from("4.5 + 3"), &mut calculator::environment::new()).unwrap(),
		7.5
	);

	assert_eq!(
		calculate(String::from("0 - 0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("4 - 7"), &mut calculator::environment::new()).unwrap(),
		-3.0
	);
	assert_eq!(
		calculate(String::from("4.5 - 3"), &mut calculator::environment::new()).unwrap(),
		1.5
	);
}

#[test]
fn test_04_multiplicative() {
	assert_eq!(
		calculate(String::from("0 * 0"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("4 * 3"), &mut calculator::environment::new()).unwrap(),
		12.0
	);
	assert_eq!(
		calculate(String::from("4.5 * 3"), &mut calculator::environment::new()).unwrap(),
		13.5
	);

	assert_eq!(
		calculate(String::from("0 / 1"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("12 / 3"), &mut calculator::environment::new()).unwrap(),
		4.0
	);
	assert_eq!(
		calculate(String::from("4.5 / 3"), &mut calculator::environment::new()).unwrap(),
		1.5
	);

	assert_eq!(
		calculate(String::from("0 % 1"), &mut calculator::environment::new()).unwrap(),
		0.0
	);
	assert_eq!(
		calculate(String::from("11 % 3"), &mut calculator::environment::new()).unwrap(),
		2.0
	);
	assert_eq!(
		calculate(String::from("4.5 % 3"), &mut calculator::environment::new()).unwrap(),
		1.5
	);
}

#[test]
fn test_05_operation_order() {
	assert_eq!(
		calculate(
			String::from("3 + 4 * 5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		23.0
	);
	assert_eq!(
		calculate(
			String::from("3 * 4 + 5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		17.0
	);

	assert_eq!(
		calculate(
			String::from("3 + -4 * 5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		-17.0
	);
	assert_eq!(
		calculate(
			String::from("3 + -4 * -5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		23.0
	);
}

#[test]
fn test_06_brackets() {
	assert_eq!(
		calculate(
			String::from("(3 + 4) * 5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		35.0
	);
	assert_eq!(
		calculate(
			String::from("3 * (4 + 5)"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		27.0
	);

	assert_eq!(
		calculate(
			String::from("3 + -(4 * 5)"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		-17.0
	);
	assert_eq!(
		calculate(
			String::from("3 + -(4 * -5)"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		23.0
	);
	assert_eq!(
		calculate(
			String::from("(3 + -4) * 5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		-5.0
	);
	assert_eq!(
		calculate(
			String::from("(3 + -4) * -5"),
			&mut calculator::environment::new()
		)
		.unwrap(),
		5.0
	);
}
