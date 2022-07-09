use std::{collections::HashMap};

fn is_seperator(c: char) -> bool {
  if c == '[' || c == '\'' || c == ']' || c == ',' || c == ' ' {
    return true;
  }
  false
}

pub fn detect_route(route: String) -> Result<String, String> {
  let mut letter_counts: HashMap<&str, i32> = HashMap::new();   // [str: count]: the number of each regions.
  let mut start_end: HashMap<&str, &str> = HashMap::new();      // [start, end]: the pair of start, end regions.
  
  let mut start: &str = "";
  let mut end: &str = "";

  /*---------------- Find possible starting and ending points --------------------*/
  let v_first: Vec<&str> = route.as_str().split(|c: char| is_seperator(c)).collect();
  let mut v: Vec<&str> = Vec::new();

  let mut index = 0;
  let mut t_start = "";
  for c in v_first.clone() {
    if !c.is_empty() {
      v.push(c);
      if index == 0 {
        t_start = c;
      }
      if index == 1 {
        start_end.insert(t_start, c);
      }
      index = (index + 1) % 2;
    }
  }

  for c in v.clone() {
    if let Some(x) = letter_counts.get_mut(&c) {
      *x = *x + 1;
    } else {
      letter_counts.insert(c, 1);
    }
  }

  let mut count_1 = 0;
  for (key, value) in &letter_counts {
    if *value % 2 == 1 {
      count_1 = count_1 + 1;
      if let Some(_x) = start_end.get_mut(key) {
        start = key;
      } 
      else {
        end = key;
      }
    }
  }
  if count_1 != 2 {
    return Err("Not valid path1".to_string());
  }

  let mut cur = start;
  loop {
    if let Some(x) = start_end.get_mut(cur) {
      cur = x;
    }
    else {
      break;
    }
  }
  if cur != end {
    return Err("Not valid path2".to_string());
  }
  
  Ok(format!("[{:?}, {:?}]", start, end))
}
