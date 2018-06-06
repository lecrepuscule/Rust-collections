mod mean_median_mode;

fn main() {
	let list_of_nums = vec![21.2, 3.4, 0.0, 309.3];
    println!("the mean is {:?}", mean_median_mode::get_mean(&list_of_nums));
    println!("the median is {:?}", mean_median_mode::get_median(&list_of_nums));
}
