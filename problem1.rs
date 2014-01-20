fn main() {
	let mut number = 0;
	let mut sum = 0;

	while number < 1000 {
		if number % 3 == 0 || number % 5 == 0 {
			sum += number;
		}
		number += 1;
	}

	println!("{:d}", sum);
}
