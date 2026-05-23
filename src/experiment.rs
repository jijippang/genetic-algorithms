
use std::time::Instant;
use log::{info};

use crate::individual::Individual;
use crate::operators::{Operator, Operate};




pub type Population = Vec<Individual>;



#[derive(Debug, Default)]
pub struct Experiment
{
    population: Population,
    curr_best_individual: Individual,
    curr_iter_cnt: usize,
    curr_duration_sec: f64,
    curr_objective_val: f64,
    start_time: Option<Instant>,
    exit_criteria: ExperimentExitCriteria,
    seed: u64,
    operator: Operator,
}

impl Experiment
{
    pub fn run(&mut self) -> Result<ExperimentResult, Box<dyn std::error::Error>>
    {
        // Mark down the specific point in time where this experiment run was started
        self.start_time = Some(Instant::now());

        match self.exit_criteria
        {
            ExperimentExitCriteria::IterationCount(exit_iter_cnt) => 
            {
                while self.curr_iter_cnt < exit_iter_cnt
                {
                    self.step();
                }
            },
            ExperimentExitCriteria::DurationSeconds(exit_duration_sec) => 
            {
                while self.curr_duration_sec < exit_duration_sec
                {
                    self.step();
                }
            },
            ExperimentExitCriteria::ObjectiveThreshold(exit_obj_thresh) => 
            {
                while self.curr_objective_val < exit_obj_thresh
                {
                    self.step();
                }
            },
        }

        // Return the experiment's result
        Ok(
            ExperimentResult {
                best_individual: self.curr_best_individual.clone(),
                final_population: self.population.clone(),
                final_iter_cnt: self.curr_iter_cnt,
                final_duration_sec: self.curr_duration_sec,
                final_objective_val: self.curr_objective_val,
            }
        )
    }

    fn step(&mut self) -> ()
    {
        // Update the current experiment attributes
        info!("Current Iteration Count: {}", self.curr_iter_cnt);
        self.curr_iter_cnt += 1;
        self.curr_duration_sec = self.start_time.expect("Start time was not initalized").elapsed().as_secs_f64();


        // Use the selected operator to run the genetic algorithm
        self.operator.operate(&self.population, self.seed);


        self.curr_objective_val = 0.0;
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
    final_iter_cnt: usize,
    final_duration_sec: f64,
    final_objective_val: f64,
}


#[derive(Debug)]
pub enum ExperimentExitCriteria
{
    IterationCount(usize),
    DurationSeconds(f64),
    ObjectiveThreshold(f64),
}

impl Default for ExperimentExitCriteria
{
    fn default() -> Self
    {
        ExperimentExitCriteria::IterationCount(Default::default())
    }
}


#[derive(Debug)]
pub struct ExperimentBuilder
{
    population: Option<Population>,
    exit_criteria: Option<ExperimentExitCriteria>,
    seed: Option<u64>,
    operator: Option<Operator>,
}

impl ExperimentBuilder
{
    pub fn new() -> Self
    {
        Self {
            population: None,
            exit_criteria: None,
            seed: None,
            operator: None,
        }
    }

    pub fn population(mut self, population: Population) -> Self
    {
        self.population = Some(population);
        self
    }

    pub fn exit_criteria(mut self, exit_criteria: ExperimentExitCriteria) -> Self
    {
        self.exit_criteria = Some(exit_criteria);
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
            exit_criteria: self.exit_criteria.unwrap_or_default(),
            seed: self.seed.unwrap_or_default(),
            operator: self.operator.unwrap_or_default(),
            ..Default::default()
        }
    }
}


