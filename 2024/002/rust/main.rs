fn clamp(val: i32, min: i32, max: i32) -> i32 {
  std::cmp::max(std::cmp::min(val, max), min)
}

fn report_is_safe(report: &str) -> bool {
  let levels: Vec<i32> = report
    .split(" ")
    .map(|level_str| level_str.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
  if levels.len() == 1 {
    return true;
  }

  // e.g 2 1 -> (2 - 1 = 1)  -> decresiong = positive direction
  //     1 2 -> (1 - 2 = -1) -> increasing = negative direction
  //     2 2 -> (2 - 2 = 0)  -> same = 0
  let mut prev_clamped_direction: Option<i32> = None;
  for i in 1..levels.len() {
    let direction = -1 * (levels[i] - levels[i-1]);
    let clamped_direction = clamp(direction, -1, 1);

    // Requirement 1: The levels are either all increasing or all decreasing.
    //   - A clamped_direction of 0 means neither increasing or decreasing.
    //   - A clamped_direction of 1 means increasing.
    //   - A clamped_direction of -1 means decreasding.
    //       - The clamped_direction needs to match the prev_clamped_direction.
    if let Some(p) = prev_clamped_direction  {
      if p != clamped_direction { return false; } // direction changed.
    }
    if clamped_direction == 0 { return false; } // not increasing or decreasing.

    // Requirement 2: Any 2 adjacent levels differ by at least 1 and at most 3.
    //  - The abs_direction must be b/w [1,3].
    let abs_direction = direction.abs();
    if abs_direction < 1 || abs_direction > 3 {
      return false;
    }
    prev_clamped_direction = Some(clamped_direction)
  }
  // The report is safe.
  true
}

fn calculate_safe_reports(input_file: &str) -> std::io::Result<()> {
  let contents = std::fs::read_to_string(input_file)?;
  let reports: Vec<_> = contents.split("\n").collect::<Vec<_>>();

  let safe_reports: i32 = reports
    .into_iter()
    .filter(|report| report.len() > 0)
    .map(|report| if report_is_safe(report) { 1 } else { 0 })
    .sum();
  
  println!("Safe reports: {}", safe_reports);
  Ok(())
}

fn main () {
  match calculate_safe_reports("2024/002/input.txt") {
    Err(e) => {
      eprintln!("Error calcualting safe reports: {:?}", e);
    },
    _ => {}
  }
}