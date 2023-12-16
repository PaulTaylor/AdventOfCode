#[allow(clippy::must_use_candidate)]
pub fn grid_string(grid: &[Vec<char>]) -> String {
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
