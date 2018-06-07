pub fn pig_latin(input_string: &String) -> String {
	let vowels = ['a', 'e', 'i', 'o', 'u'];
	let mut pig_latin_string = String::new();
	let mut pig_latin_suffix = String::from("-");

	for (i, c) in input_string.chars().enumerate() {
		if i == 0 {
			match vowels.iter().position(|&s| s == c) {
				None => {
					pig_latin_suffix.push_str(&c.to_string());
					pig_latin_suffix.push_str("ay");
					continue;
				},
				_ => {
					pig_latin_suffix.push_str("hay");
				}
			}
		}
		pig_latin_string.push(c);
	}
	pig_latin_string + &pig_latin_suffix
}