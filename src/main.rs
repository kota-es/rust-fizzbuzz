mod fizzbuzz;
use fizzbuzz::FizzBuzz;

fn main() {
  (1..=15).map(|x| -> FizzBuzz {
    x.into()
  }).map(|x| -> String {
    x.to_string()
  }).for_each(|x| println!("{x}"))
}

#[test]
fn test() {
  let test_target: Vec<String> = (1..=15)
    .map(|x| -> FizzBuzz {
      x.into()
    })
    .map(|x| -> String {
      x.to_string()
    })
    .collect();

  assert_eq!(
    test_target,
    vec![
      "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
      "14", "FizzBuzz",
    ]
  );
}
