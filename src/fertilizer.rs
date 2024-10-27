struct MappingInterval {
  source_start: i64,
  dest_start: i64,
  len: i64,
}

impl MappingInterval {
  fn parse(line: &str) -> Self {
    let mut nums = line.split_whitespace();

    Self {
      dest_start: nums.next().unwrap().parse().unwrap(),
      source_start: nums.next().unwrap().parse().unwrap(),
      len: nums.next().unwrap().parse().unwrap(),
    }
  }
  fn apply(&self, input: i64) -> Option<i64> {
    if input < self.source_start {
      None
    } else if input - self.source_start < self.len {
      Some(self.dest_start + (input - self.source_start))
    } else {
      None
    }
  }

  fn contains_val(&self, input: i64) -> bool {
    input >= self.source_start && input - self.source_start <= self.len
  }
}

struct MappingTable {
  source_type: String,
  dest_type: String,
  ranges: Vec<MappingInterval>, // We assume this is sorted source_start, asc
}

impl MappingTable {
  fn parse(paragraph: &str) -> Self {
    let mut lines = paragraph.lines();

    let title = lines.next().unwrap().trim().split(" ").next().unwrap();
    let mut src_to_dest = title.split("-");
    let source_type = src_to_dest.nth(0).unwrap().to_string();
    let dest_type = src_to_dest.nth(1).unwrap().to_string();

    let mut ranges: Vec<MappingInterval> = lines.map(|line| MappingInterval::parse(line)).collect();

    ranges.sort_by_key(|interval| interval.source_start);

    Self {
      source_type,
      dest_type,
      ranges,
    }
  }
  fn apply(&self, input: i64) -> i64 {
    let _interval = self.find_interval(input);

    if let Some(mapped) = self
      .find_interval(input)
      .and_then(|interval| interval.apply(input))
    {
      mapped
    } else {
      input
    }
  }

  // TODO: This does not support the case of overlapping intervals.
  // If overlapping cases exist in the dataset, then need to refactor this.
  fn find_interval(&self, input: i64) -> Option<&MappingInterval> {
    self.ranges.iter().find(|range| range.contains_val(input))
    // match self
    //   .ranges
    //   .binary_search_by_key(&input, |interval| interval.source_start)
    // {
    //   Result::Ok(found_elem) => &self.ranges[found_elem],
    //   Result::Err(found_elem) => &self.ranges[found_elem],
    // }
  }
}

pub fn solve(input: String) -> (i64, i64) {
  let mut paragraphs = input.split("\n\n");
  // First paragraph is the seeds
  let seeds: Vec<i64> = paragraphs.next().unwrap()[7..]
    .trim()
    .split_whitespace()
    .map(|n| n.parse().unwrap())
    .collect();

  let ans_1 = paragraphs
    .map(|p| MappingTable::parse(p))
    .fold(seeds, |nums, table| {
      nums.into_iter().map(|n| table.apply(n)).collect()
    })
    .into_iter()
    .min()
    .unwrap();

  (ans_1, 0)
}

#[cfg(test)]
mod tests {
  use crate::testing::test_with_input;

  use super::solve;

  #[test]
  fn test_fertilizer_sample() {
    test_with_input("fertilizer", solve, 35i64, 0i64)
  }
}
