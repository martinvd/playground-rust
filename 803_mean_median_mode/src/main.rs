use std::collections::HashMap;

fn get_average(numbers: &[i32]) -> f32 {
  let total: i32 = numbers.into_iter().sum();
  let size = numbers.len() as f32;

  total as f32 / size
}

fn get_median(numbers: &[i32]) -> i32 {
  let mut new_numbers = numbers.to_owned();

  new_numbers.sort();

  new_numbers[new_numbers.len() / 2]
}

fn get_mode(numbers: &[i32]) -> (i32, i32) {
  let mut list = HashMap::new();

  for number in numbers {
    let count = list.entry(*number).or_insert(0);
    *count += 1;
  }

  let max = list.iter().max_by_key(|&(_, count)| count);

  match max {
    Some(v) => (*v.0, *v.1),
    None => (0, 0),
  }
}

fn main() {
  let numbers = vec![1, 5, 2, 4, 4, 3, 2, 1, 2];

  println!("Average: {}", get_average(&numbers));
  println!("Median: {}", get_median(&numbers));
  println!("Mode: {}", get_mode(&numbers).0);
}
