use rand::{random, Rng, thread_rng};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::individual::Individual;

type Population = Vec<Individual>;

fn gen_pop(pop_size: usize, chromosome_size: usize, min_val: f64, max_val: f64, rng: &mut ThreadRng) -> Population {
    (0..pop_size).map(|_| Individual {
        chromosome: (0..chromosome_size).map(|_| rng.gen_range(min_val..max_val)).collect(),
        fitness: 0.0
    }).collect()
}

fn evaluate(pop: &mut Population, fitness: fn(&Vec<f64>) -> f64) {
    pop.iter_mut().for_each(|ind| {
        ind.fitness = fitness(&ind.chromosome);
    });
}

fn rank(pop: &mut Population) {
    pop.sort_by(|ind1, ind2| ind2.fitness.partial_cmp(&ind1.fitness).unwrap());
}

fn select<'pop>(pop: &'pop Population, sum_of_fitnesses: f64, rng: &mut ThreadRng) -> &'pop Individual {
    let rnd: f64 = random();

    for ind in pop {
        let ind_selection_prob = ind.fitness / sum_of_fitnesses;

        if rnd < ind_selection_prob {
            return ind;
        }
    }

    pop.choose(&mut rng.clone()).unwrap()
}

fn crossover(par1: &Individual, par2: &Individual, chromosome_size: usize, min_val: f64, max_val: f64) -> Individual {
    Individual {
        chromosome: (0..chromosome_size).enumerate().map(|(gene_id, _)| {
            let gene = f64::from_bits(par1.chromosome[gene_id].to_bits() & par2.chromosome[gene_id].to_bits());

            if gene < min_val { return min_val }
            else if gene > max_val { return max_val }

            gene
        }).collect(),
        fitness: 0.0
    }
}

fn mutate(ind: &mut Individual, min_val: f64, max_val: f64, rng: &mut ThreadRng) {
    let rnd: f64 = random();
    let rnd_bit_pos =
    if rnd <= 0.75 { rng.gen_range(0..32) }
    else if rnd <= 0.90 { rng.gen_range(31..48) }
    else { rng.gen_range(47..64) };

    let mask = 1 << rnd_bit_pos;

    ind.chromosome.iter_mut().for_each(|gene| {
        let mut mutated_gene = f64::from_bits(gene.to_bits() ^ mask);

        if mutated_gene < min_val { mutated_gene = min_val; }
        else if mutated_gene > max_val { mutated_gene = max_val; }

        *gene = mutated_gene;
    });
}

pub fn ga(chromosome_size: usize, min_val: f64, max_val: f64, mut_rat: f64, pop_size: usize, nofit: i32, fitness: fn(&Vec<f64>) -> f64) -> Individual {
    let mut pop;
    let parents_size = pop_size / 2;
    let mut best: Individual = Individual { chromosome: vec![], fitness: 0.0 };
    let mut rng = thread_rng();

    println!("Generating initial population...");
    pop = gen_pop(pop_size, chromosome_size, min_val, max_val, &mut rng);

    println!("Generated population: {:?}", pop);

    println!("Evaluating initial population...");
    evaluate(&mut pop, fitness);
    rank(&mut pop);

    println!("Evaluated initial population: {:?}", pop);

    for i in 0..nofit {
        let parents: Vec<[&Individual; 2]>;
        let children: Vec<Individual>;

        println!("Generation {:?}:", i + 1);

        {
            let sum_of_fitnesses = pop.iter().map(|ind| ind.fitness).sum();

            println!("Selecting parents...");

            parents = (0..parents_size).map(|_| {
                let parents = [select(&pop, sum_of_fitnesses, &mut rng), select(&pop, sum_of_fitnesses, &mut rng)];

                println!("Selected parents: {:?}", parents);

                parents
            }).collect();
        }

        {
            println!("Crossing-over...");

            children = parents.iter().enumerate().map(|(couple_id, _)| {
                let parent1 = parents[couple_id][0];
                let parent2 = parents[couple_id][1];

                println!("Cross_over: {:?} {:?}", parent1, parent2);

                let child = crossover(parent1, parent2, chromosome_size, min_val, max_val);

                println!("Result: {:?}", child);

                child
            }).collect();
        }

        pop.reverse();

        pop.splice(0..children.len(), children);

        pop.reverse();

        {
            let mut mut_occurred = false;

            println!("Mutating...");
            pop.iter_mut().for_each(|ind| {
                let rnd: f64 = random();

                if rnd < mut_rat {
                    let ind_bef_mut = ind.clone();

                    mutate(ind, min_val, max_val, &mut rng);

                    println!("Mutation occurred: {:?} -> {:?}", ind_bef_mut, ind);

                    mut_occurred = true;
                }
            });

            if !mut_occurred { println!("No mutation occurred."); }
        }

        println!("Evaluating population...");
        evaluate(&mut pop, fitness);
        rank(&mut pop);

        println!("Evaluated population: {:?}", pop);

        println!("New population: {:?}", pop);

        best = pop[0].clone();

        println!("Best of generation: {:?}", best);
    }

    best
}
