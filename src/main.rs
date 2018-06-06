mod mean_median_mode;

fn main() {
	let list_of_nums = vec![21.2, 3.4, 0.0, 309.3];
	let list_of_ints = vec![3,6,43,-222,4,55,6, 55,6];
    println!("the mean is {:?}", mean_median_mode::get_mean(&list_of_nums));
    println!("the median is {:?}", mean_median_mode::get_median(&list_of_nums));
    println!("the mode is {:?}", mean_median_mode::get_mode(&list_of_ints).unwrap());
}
