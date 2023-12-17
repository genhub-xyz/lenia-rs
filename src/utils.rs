use itertools::Itertools;
use ndarray::{array, Array1, ArrayView1};

pub fn ones(rows: usize, cols: usize) -> Vec<Vec<f32>> {
    vec![vec![1.; rows]; cols]
}

pub fn zeros(rows: usize, cols: usize) -> Vec<Vec<f32>> {
    vec![vec![0.; rows]; cols]
}

pub fn range(start: f32, step: f32, n: usize) -> Vec<f32> {
    let mut val = start;
    let mut arr = vec![start];
    for i in 0..n {
        val += step;
        arr.push(val);
    }
    arr
}

pub fn norm(x: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut mat = Vec::new();
    for i in 0..x.len() {
        let mut arr = Vec::new();
        for j in 0..x.len() {
            let mut x_i = x[i].clone();
            let x_j = x[j].clone();
            x_i.extend_from_slice(&x_j);
            let lhs = Array1::from(x_i);
            let norm = lhs.dot(&lhs).sqrt();
            arr.push(norm);
        }
        mat.push(arr);
    }
    mat
}
