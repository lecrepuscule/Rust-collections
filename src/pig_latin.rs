pub fn pig_latin(input_string: &String) -> String {
	let vowels = ['a', 'e', 'i', 'o', 'u'];
	let mut pig_latin_string = String::new();
	let mut pig_latin_suffix = String::from("-");

	for (i, c) in input_string.chars().enumerate() {
		if i == 0 {
			let is_vowel = vowels.iter().position(|&s| s == c);
			if is_vowel.is_none() {
				pig_latin_suffix.push_str(&c.to_string());
				pig_latin_suffix.push_str("ay");
				continue;
			} else {
				pig_latin_suffix.push_str("hay");
			}
		}
		pig_latin_string.push(c);
	}
	pig_latin_string + &pig_latin_suffix
}