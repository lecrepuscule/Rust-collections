use std::cmp::Ordering;

pub fn get_mean(list: &Vec<f32>) -> Vec<f32> {
	let mut sum: f32 = 0.0;

	let mut results: Vec<f32> = Vec::new();

	for num in list {
		sum += num;
	}

	results.push(sum as f32/ list.len() as f32);
	results
}


pub fn get_median(list: &Vec<f32>) -> f32 {

	let mut v: Vec<_> = list.iter().map(|v| NonNan::new(*v).unwrap()).collect();
	v.sort();
	let sorted_list: Vec<f32> = v.iter().map(|v| v.0).collect();



	if list.len() % 2 == 0 {
		let middle_index = list.len() / 2 ;
		(sorted_list[middle_index] + sorted_list[middle_index - 1]) / 2.0
	} else {
		let middle_index = (list.len() - 1) / 2;
		sorted_list[middle_index]
	}

}

#[derive(PartialEq,PartialOrd)]
struct NonNan(f32);

impl NonNan {
    fn new(val: f32) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
