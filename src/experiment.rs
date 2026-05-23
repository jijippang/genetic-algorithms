
use log::{info};

use crate::individual::Individual;
use crate::operators::Operator;




pub type Population = Vec<Individual>;



#[derive(Debug, Default)]
pub struct Experiment
{
    population: Population,
    curr_best_individual: Individual,
    curr_iter_cnt: usize,
    max_iter_cnt: usize,
    seed: u64,
    operator: Operator,
}


impl Experiment
{
    pub fn run(&mut self) -> Result<ExperimentResult, Box<dyn std::error::Error>>
    {
        while self.curr_iter_cnt < self.max_iter_cnt
        {
            self.step();
        }

        // Return the experiment's result
        Ok(
            ExperimentResult {
                best_individual: self.curr_best_individual.clone(),
                final_population: self.population.clone(),
            }
        )
    }

    fn step(&mut self) -> ()
    {
        info!("Current Iteration Count: {}", self.curr_iter_cnt);
        self.curr_iter_cnt += 1;


    }

    pub fn size(&self) -> usize
    {
        self.population.len()
    }
}


#[derive(Debug)]
pub struct ExperimentResult
{
    best_individual: Individual,
    final_population: Population,
}


#[derive(Debug)]
pub struct ExperimentBuilder
{
    population: Option<Population>,
    curr_best_individual: Option<Individual>,
    curr_iter_cnt: Option<usize>,
    max_iter_cnt: Option<usize>,
    seed: Option<u64>,
    operator: Option<Operator>,
}

impl ExperimentBuilder
{
    pub fn new() -> Self
    {
        Self {
            population: None,
            curr_best_individual: None,
            curr_iter_cnt: None,
            max_iter_cnt: None,
            seed: None,
            operator: None,
        }
    }

    pub fn population(mut self, population: Population) -> Self
    {
        self.population = Some(population);
        self
    }

    pub fn curr_best_individual(mut self, curr_best_individual: Individual) -> Self
    {
        self.curr_best_individual = Some(curr_best_individual);
        self
    }

    pub fn curr_iter_cnt(mut self, curr_iter_cnt: usize) -> Self
    {
        self.curr_iter_cnt = Some(curr_iter_cnt);
        self
    }

    pub fn max_iter_cnt(mut self, max_iter_cnt: usize) -> Self
    {
        self.max_iter_cnt = Some(max_iter_cnt);
        self
    }

    pub fn seed(mut self, seed: u64) -> Self
    {
        self.seed = Some(seed);
        self
    }

    pub fn operator(mut self, operator: Operator) -> Self
    {
        self.operator = Some(operator);
        self
    }

    pub fn build(self) -> Experiment
    {
        Experiment {
            population: self.population.unwrap_or_default(),
            curr_best_individual: self.curr_best_individual.unwrap_or_default(),
            curr_iter_cnt: self.curr_iter_cnt.unwrap_or_default(),
            max_iter_cnt: self.max_iter_cnt.unwrap_or_default(),
            seed: self.seed.unwrap_or_default(),
            operator: self.operator.unwrap_or_default(),
        }
    }
}

