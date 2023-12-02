#[cfg(test)]
mod test {
    use candle_core::{Device, Tensor};
    use rand::Rng;

    const NUM_ITER: usize = 100;

    #[test]
    fn tensor_test() {
        let width = 64;
        let height = 64;
        let mut rng = rand::thread_rng();
        let mut cells = (0..width * height)
            .into_iter()
            .map(|_| rng.gen::<bool>().into())
            .collect::<Vec<f64>>();

        for i in 0..NUM_ITER {
            let image =
                Tensor::from_vec(cells.clone(), (1, 1, width, height), &Device::Cpu).unwrap();

            let filter = [[1., 1., 1.], [1., 0., 1.], [1., 1., 1.]];
            let kernel = Tensor::new(&[[filter]], &Device::Cpu).unwrap();
            let res = image.conv2d(&kernel, 1, 1, 1, 1).unwrap();

            let res_flatten = res.flatten_to(3).unwrap().to_vec1::<f64>().unwrap();
            cells = cells
                .iter()
                .zip(res_flatten)
                .map(|(x, y)| {
                    let is_alive = *x == 1.;
                    let num_neighbour_alive = y;

                    if is_alive && (num_neighbour_alive == 2. || num_neighbour_alive == 3.) {
                        return 1.; // alive
                    }
                    if !is_alive && num_neighbour_alive == 3. {
                        return 1.;
                    }

                    0.
                })
                .collect()
        }
    }
}
