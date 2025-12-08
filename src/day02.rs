type Input = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
  input.split(',').map(|range| {
    let mut iter = range.split('-');
    (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
  }).collect()
}

pub fn p1(input: &Input) -> u64 {
  let check: fn(&u64) -> u64 = |x| {
    let s = x.to_string();
    let n = s.len();
    if s[0..n/2] == s[n/2..] {
      *x
    } else {
      0
    }
  };

  let mut ans = 0;
  for &(l, r) in input.iter() {
    for i in l..r+1 {
      ans += check(&i);
    }
  }
  ans
}

pub fn p2(input: &Input) -> u64 {
  let check: fn(&u64) -> u64 = |x| {
    let s = x.to_string();
    let n = s.len();
    for rep in 2..n+1 {
      if n % rep != 0 {
        continue;
      }

      let mut ok = true;
      for i in 0..rep-1 {
        if s[i*n/rep..(i+1)*n/rep] != s[(i+1)*n/rep..(i+2)*n/rep] {
          ok = false;
        }
      }

      if ok {
        return *x;
      }
    }
    0
  };

  let mut ans = 0;
  for &(l, r) in input.iter() {
    for i in l..r+1 {
      ans += check(&i);
    }
  }
  ans
}