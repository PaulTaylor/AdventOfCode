use std::collections::HashMap;

#[allow(clippy::must_use_candidate)]
pub fn _grid_string(grid: &[Vec<char>]) -> String {
    let s: String = grid
        .iter()
        .map(|row| -> String {
            let mut s: String = row.iter().collect();
            s.push('\n');
            s
        })
        .collect();

    s.trim_end().to_string()
}

#[allow(clippy::must_use_candidate, clippy::missing_panics_doc)]
pub fn _sparse_grid_string<S: ::std::hash::BuildHasher>(
    grid: &HashMap<(isize, isize), char, S>,
    highlight_pos: Option<(isize, isize)>,
) -> String {
    let (max_x, max_y) = grid
        .keys()
        .copied()
        .reduce(|(mx, my), (x, y)| (std::cmp::max(mx, x), std::cmp::max(my, y)))
        .unwrap();

    let out_lines: Vec<_> = (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| {
                    grid.get(&(x, y)).unwrap_or(match highlight_pos {
                        Some(hp) if (hp == (x, y)) => &'@',
                        _ => &'.',
                    })
                })
                .collect::<String>()
        })
        .collect();

    out_lines.join("\n")
}
