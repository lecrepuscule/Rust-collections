mod mean_median_mode;

fn main() {
	let list_of_nums = vec![21.2, 3.4, 0.0, 309.3];
    println!("the result is {:?}", mean_median_mode::mean_median_mode(&list_of_nums));
}
