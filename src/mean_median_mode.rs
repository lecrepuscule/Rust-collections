
pub fn mean_median_mode(list: &Vec<f32>) -> Vec<f32> {
	let mut mean: f32 = 0.0;
;

	let mut results: Vec<f32> = Vec::new();

	for num in list {
		mean += num;
	}

	results.push(mean as f32/ list.len() as f32);
	results
}

