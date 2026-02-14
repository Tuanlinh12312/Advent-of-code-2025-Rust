use grid::*;
type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>()
        .into()
}

const DIRS: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

pub fn p1(input: &Input) -> usize {
    input
        .indexed_iter()
        .filter(|&((i, j), &c)| {
            if c != '@' {
                false
            } else {
                DIRS.iter()
                    .filter(|&&(di, dj)| input.get(i as isize + di, j as isize + dj) == Some(&'@'))
                    .count()
                    < 4
            }
        })
        .count()
}

pub fn p2(input: &Input) -> usize {
    let mut ans = 0;
    let mut m_input = input.clone();
    loop {
        let to_remove: Vec<_> = m_input
        .indexed_iter()
        .filter(|&((i, j), &c)| {
            if c != '@' {
                false
            } else {
                DIRS.iter()
                    .filter(|&&(di, dj)| m_input.get(i as isize + di, j as isize + dj) == Some(&'@'))
                    .count()
                    < 4
            }
        })
        .map(|((i, j), _)| (i, j))
        .collect();

        if to_remove.is_empty() {
            return ans;
        } else {
            ans += to_remove.len();

            for (i, j) in to_remove {
                m_input[(i, j)] = '.';
            } 
        }
    }
}
