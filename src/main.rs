use std::env;
use crate::ga::{ga, Individual};

mod ga;

fn fitness(input: &Vec<f64>) -> f64 {
    let x = input[0];

    (12.0 * x.powi(5) - 975.0 * x.powi(4) + 28000.0 * x.powi(3) - 345000.0 * x.powi(2) + 1800000.0 * x) / 1000000.0
}

fn main() {
    let chromosome_size = env!("CHROMOSOME_SIZE").parse::<usize>().unwrap();
    let min_val = env!("MIN_VAL").parse::<f64>().unwrap();
    let max_val = env!("MAX_VAL").parse::<f64>().unwrap();

    let mut_rat = env!("MUT_RAT").parse::<f64>().unwrap();
    let pop_size = env!("POP_SIZE").parse::<usize>().unwrap();
    let nofit = env!("NOFIT").parse::<i32>().unwrap();
    let nofr = env!("NOFR").parse::<i32>().unwrap();

    let mut last_best = ga(chromosome_size, min_val, max_val, mut_rat, pop_size, nofit, fitness);
    let mut best = Individual { chromosome: vec![], fitness: 0.0 };

    for i in 0..nofr {
        if best.fitness <= last_best.fitness {
            println!("Run {:?}", i + 1);

            best = last_best.clone();
            last_best = ga(chromosome_size, min_val, max_val, mut_rat, pop_size, nofit, fitness);

            println!("Best of run: {:?}", best);
        } else { break; }
    }

    println!("Best solution: {:?}", best);

    println!("Program execution finished.");
}
