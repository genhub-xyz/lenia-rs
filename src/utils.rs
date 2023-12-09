pub fn ones(rows: usize, cols: usize) -> Vec<Vec<f32>> {
    vec![vec![1.; rows]; cols]
}

pub fn zeros(rows: usize, cols: usize) -> Vec<Vec<f32>> {
    vec![vec![0.; rows]; cols]
}
