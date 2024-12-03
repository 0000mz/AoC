fn clamp(val: i32, min: i32, max: i32) -> i32 {
  std::cmp::max(std::cmp::min(val, max), min)
}

fn level_permutations(levels: &[i32]) -> Vec<Vec<i32>> {
  let mut perms: Vec<Vec<i32>> = Vec::new();
  for i in 0..levels.len() {
    let mut p: Vec<usize> = (0..i).collect();
    p.append(&mut (i+1..levels.len()).collect());
    // p contains the indices that should be included in the permutaiton.
    // Resolve the indices to the actual values.
    perms.push(p
      .into_iter()
      .map(|idx| levels[idx])
      .collect()
    );
  }
  perms
}

fn single_report_is_safe(levels: &[i32]) -> bool {
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

// Calculate if the report is safe. A report is safe if both are true:
//   1. The levels are either all increasing or all decreasing.
//   2. Any 2 adjacent levels differ by at least 1 and at most 3.
//
//   If `dampen` is true, a single bad level can be tolerated.
fn report_is_safe(levels: &[i32], dampen: bool) -> bool {
  if levels.len() == 1 {
    return true;
  }
  (if dampen { level_permutations(levels) } else { vec![levels.to_vec()] })
    .into_iter()
    .map(|l| if single_report_is_safe(&l[..]) { 1 } else { 0 })
    .sum::<i32>() > 0
}

fn calculate_safe_reports(input_file: &str) -> std::io::Result<()> {
  let contents = std::fs::read_to_string(input_file)?;
  let reports: Vec<Vec<i32>> = contents
    .split("\n")
    .filter(|report| report.len() > 0)
    .map(|report| {
      report
        .split(" ")
        .map(|level_str| level_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    })
    .collect::<Vec<_>>();

  for dampen in [false, true] {
    let safe_reports: i32 = reports
      .iter()
      .map(|report| if report_is_safe(&report[..], dampen) { 1 } else { 0 })
      .sum();  
    println!("Safe reports [dampen? {}]: {}", dampen, safe_reports);
  }
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