use grid::*;
type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
        .into()
}

pub fn p1(grid: &Input) -> u32 {
    let mut ans = 0;
    let (rows, cols) = grid.size();
    let mut has_beam = vec![rows; cols];

    let splitters = grid
        .indexed_iter()
        .filter(|(_, c)| c == &&'^')
        .map(|(coord, _)| coord)
        .collect::<Vec<_>>();
    let (sx, sy) = grid
        .indexed_iter()
        .filter(|(_, c)| c == &&'S')
        .map(|(coord, _)| coord)
        .nth(0)
        .unwrap();

    has_beam[sy] = sx + 1;
    for (x, y) in splitters {
        if has_beam[y] <= x {
            ans += 1;
            has_beam[y] = rows;

            if y >= 1 {
                has_beam[y - 1] = x + 1;
            }
            if y + 1 <= rows {
                has_beam[y + 1] = x + 1;
            }
        }
    }
    ans
}

pub fn p2(grid: &Input) -> usize {
    let mut ans = 0;
    let (rows, cols) = grid.size();
    let mut has_beam = vec![0; cols];

    let splitters = grid
        .indexed_iter()
        .filter(|(_, c)| c == &&'^')
        .map(|(coord, _)| coord)
        .collect::<Vec<_>>();
    let (sx, sy) = grid
        .indexed_iter()
        .filter(|(_, c)| c == &&'S')
        .map(|(coord, _)| coord)
        .nth(0)
        .unwrap();

    has_beam[sy] = 1;
    for (x, y) in splitters {
        if y >= 1 {
            has_beam[y - 1] += has_beam[y];
        }
        if y + 1 <= rows {
            has_beam[y + 1] += has_beam[y];
        }

        has_beam[y] = 0;
    }
    has_beam.iter().sum()
}
