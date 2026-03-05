use std::fs;
use std::fs::File;
use std::io::Write;
use rand::Rng;

use std::time::Instant;

fn generate_bernoulli(p: f64, n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| if rng.gen_range(0.0..=1.0) < p {1} else {0})
        .collect()
}

fn main() -> std::io::Result<()> {

    let now = Instant::now();

    fs::create_dir_all("results")?;

    let mut file = File::create("results/wyniki.csv")?;

    writeln!(file, "p,k,mean,var,err_x,err_var")?;



    let p_arr = vec![0.1, 0.5, 0.9];


    let n = 10_usize.pow(7);

    let samples: Vec<Vec<u8>> = p_arr
        .iter()
        .map(|&p| generate_bernoulli(p, n))
        .collect();

    // println!("p,k,mean,var,err_x,err_var");

    for (i, sample) in samples.iter().enumerate() {

        let p = p_arr[i];

        for k in 2..=7 {
            let n: usize = 10_usize.pow(k);

            let n_samples = &sample[..n];

            let mean = n_samples.iter().map(|&x| x as u64).sum::<u64>() as f64 / n as f64;

            // let mean_sq = n_samples.iter()
            //     .map(|&x| (x * x) as i32)
            //     .sum::<i32>() as f64 / n as f64;
            
            let mean_sq = mean;

            let sq_mean = mean.powi(2);

            let err_x = ((mean - p) / p).abs();

            let var = (mean_sq - sq_mean) / n as f64;

            let var_theo = p * (1.0 - p) / n as f64;

            let err_var = ((var - var_theo) / var_theo).abs();

            // println!("{p},{k},{mean},{var},{err_x},{err_var}");

            writeln!(file, "{},{},{},{},{},{}", p, k, mean, var, err_x, err_var)?;

        }


    }

    let elapsed = now.elapsed();

    println!("Wykonano w {:.2?}", elapsed);
    println!("Wyniki zapisano do pliku 'wyniki.csv'...");

    Ok(())

}
