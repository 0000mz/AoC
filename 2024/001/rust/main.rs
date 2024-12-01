fn parse_line(line: &str) -> Option<(i32, i32)> {
  let parts: Vec<_> = line.split(' ')
    .filter(|el| el.len() > 0)
    .collect::<Vec<_>>();
  if parts.len() == 0 {
    return None
  }
  assert_eq!(parts.len(), 2, "{}", line);
  let left = parts[0].to_string().parse::<i32>().unwrap();
  let right = parts[1].to_string().parse::<i32>().unwrap();
  Some((left, right))
}

fn compute(input_file: &str) -> std::io::Result<()> {
  let contents = std::fs::read_to_string(input_file)?;
  let lines: Vec<_> = contents.split("\n").collect::<_>();

  let parsed_lines: Vec<(i32, i32)> = lines
    .into_iter()
    .map(|line| parse_line(line))
    .filter(|res| res.is_some())
    .map(|parsed| parsed.unwrap())
    .collect::<_>();
  
  let mut left: Vec<i32> = parsed_lines
    .iter()
    .map(|(left, _)| left.clone())
    .collect::<_>();
  let mut right: Vec<i32> = parsed_lines
    .into_iter()
    .map(|(_, right)| right)
    .collect::<_>();
  
  left.sort();
  right.sort();

  let diffs = {
    let mut d = vec![];
    for i in 0..left.len() {
      let val = left[i] - right[i];
      d.push(val.abs());
    }
    d
  };

  let distance_sum: i32 = diffs.into_iter().sum();
  println!("Distance sum: {}", distance_sum);
  Ok(())
}

fn main() {
  match compute("2024/001/input.txt") {
    Err(err) => {
      eprintln!("Error: {:?}\n", err);
    },
    _ => {}
  }
}
