type Input = (Vec<(usize, usize)>, Vec<usize>);

pub fn parse(input: &str) -> Input {
    (input
        .lines()
        .filter(|&line| !line.parse::<usize>().is_ok() && line.split_once('-').is_some())
        .map(|line| {
            let (s1, s2) = line.split_once('-').unwrap();
            (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())})
        .collect(),
    input
        .lines()
        .filter(|&line| line.parse::<usize>().is_ok())
        .map(|line| line.parse::<usize>().unwrap())
        .collect())
}

pub fn p1(input: &Input) -> usize {
    let mut ans = 0;
    let (ranges, nums) = input;
    for num in nums {
        let mut cnt = 0;
        for (l, r) in ranges {
            if l <= num && num <= r {
                cnt = 1;
                break;
            }
        }
        ans += cnt;
    }
    ans
}

pub fn p2(input: &Input) -> isize {
    let (ranges, _) = input;
    let mut sorted_ranges = ranges.clone();
   sorted_ranges.sort();
    
    let mut ans = 0;
    let mut last_r = 0;
    for (l, r) in sorted_ranges {
        ans += std::cmp::max(0, r as isize - std::cmp::max(l as isize - 1, last_r));
        last_r = std::cmp::max(last_r, r as isize);
    }
    ans
}
