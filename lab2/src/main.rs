use std::fs;
use std::fs::File;
use std::io::Write;
use rand::Rng;

use std::time::Instant;

trait Generator {
    fn generate_sample(&mut self) -> f64;

    fn give_details(&self) -> String;

    fn pdf(&self, x: f64) -> f64 {
        4.0 / 5.0 * (1.0 + x - x.powi(3))
    }

    fn cdf(&self, x: f64) -> f64 {
        4.0 / 5.0 * (x + x.powi(2) / 2.0 - x.powi(4) / 4.0)
    }

    fn generate_expected_sample(&self, x1: f64, x2: f64) -> f64 {
        self.cdf(x2) - self.cdf(x1)
    }
}

struct ComplexDistr;

impl ComplexDistr {
    // fn new() {}
}

impl Generator for ComplexDistr {

    fn generate_sample(&mut self) -> f64 {
        let g1: f64 = 4.0/5.0;

        let mut rng = rand::thread_rng();

        let u1 = rng.gen_range(0.0..=1.0);
        let u2 = rng.gen_range(0.0..=1.0);

        let x: f64 = if u1 <= g1 {u2} else {(1.0 - (1.0 - u2).sqrt()).sqrt()};

        x
    }

    fn give_details(&self) -> String {
        String::from("complex")
    }
}

struct Metropolis {
    last_sample: f64,
    delta: f64,
}

impl Metropolis {
    fn new(delta: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x0 = rng.gen_range(0.0..=1.0);

        Self {
            last_sample: x0,
            delta: delta,
        }
    }
}

impl Generator for Metropolis {
    fn generate_sample(&mut self) -> f64 {
        let mut rng = rand::thread_rng();

        let u1 = rng.gen_range(0.0..=1.0);
        let u2 = rng.gen_range(0.0..=1.0);
        
        let x_new = self.last_sample + (2.0*u1 - 1.0) * self.delta;

        let p = self.pdf(x_new) / self.pdf(self.last_sample);

        let p_acc = p.min(1.0);

        let x_new_in_range = x_new <= 1.0 && x_new >= 0.0;

        let x_new = if x_new_in_range & (u2 <= p_acc) {x_new} else {self.last_sample};

        self.last_sample = x_new;

        x_new
    }

    fn give_details(&self) -> String {
        format!("metropolis_d_{}", self.delta)
    }
}

struct Elimination {
    m_param: f64,
}

impl Elimination {
    fn new(m: f64) -> Self {
        Self {
            m_param : m,
        }
    }
}

impl Generator for Elimination {
    fn generate_sample(&mut self) -> f64 {
        let mut rng = rand::thread_rng();
        let m = self.m_param;

        loop {
            let u1 = rng.gen_range(0.0..=1.0);
            let g2 = rng.gen_range(0.0..=m);

            if g2 <= self.pdf(u1) {
                return u1
            }
        }
    }

    fn give_details(&self) -> String {
        String::from("elimination")
    }
}

fn generate_data(g: &mut dyn Generator, n: usize, bins: usize) -> std::io::Result<()> {
    
    let now = Instant::now();

    let save_path = String::from("results");

    fs::create_dir_all(&save_path)?;

    let filename = format!("{}/wyniki_{}.csv", save_path, g.give_details());

    let mut file = File::create(&filename)?;

    let mut histogram = vec![0u32; bins];

    for _ in 0..n {
        let x = g.generate_sample();

        let bin = (x * bins as f64).floor() as usize;

        histogram[bin] += 1;
    }

    for counts in histogram {
        writeln!(file, "{}", counts)?;
    }


    let elapsed = now.elapsed();

    println!("Wykonano w {:.2?}", elapsed);

    Ok(())
}

fn generate_expected_data(g: &mut dyn Generator, x_min: f64, x_max: f64, bins: usize) -> std::io::Result<()> {

    let now = Instant::now();

    let save_path = String::from("results");

    fs::create_dir_all(&save_path)?;

    let filename = format!("{}/wyniki_expected.csv", save_path);

    let mut file = File::create(&filename)?;

    let step = (x_max - x_min) / bins as f64;

    for bin in 0..bins {
        let x1 = x_min + step * bin as f64;
        let x2 = x_min + step * (bin as f64 + 1.0);

        let pi = g.generate_expected_sample(x1, x2);

        writeln!(file, "{}", pi)?;
    }

    let elapsed = now.elapsed();

    println!("Wykonano w {:.2?}", elapsed);

    Ok(())
}


fn main() {

    let n = 10_usize.pow(6);

    let bins = 10_usize;

    let mut generators: Vec<Box<dyn Generator>> = vec![
        Box::new(ComplexDistr),
        Box::new(Metropolis::new(0.5)),
        Box::new(Metropolis::new(0.05)),
        Box::new(Elimination::new(1.15)),
    ];

    for generator in &mut generators {
        generate_data(generator.as_mut(), n, bins).expect("Error while generating data or saving it to file");
    }

    let x_min = 0.0;
    let x_max = 1.0;

    generate_expected_data(generators[0].as_mut(), x_min, x_max, bins).expect("Error while generating expected data or saving it to file");
    
}