
use rand::RngExt;
use rand_distr::{Distribution, StandardNormal};

use crate::individual::Individual;
use crate::population::Population;






pub trait Operate
{
    fn operate(&self, population: &mut Population, rng: &mut impl RngExt) -> ();
}


#[derive(Debug)]
pub enum Operator
{
    Selection(SelectionOperator),
    Crossover(CrossoverOperator),
    Mutation(MutationOperator),
}

impl Default for Operator
{
    fn default() -> Self
    {
        Operator::Selection(
            SelectionOperator {

            }
        )
    }
}

impl Operate for Operator
{
    fn operate(&self, population: &mut Population, rng: &mut impl RngExt) -> ()
    {
        match self
        {
            Self::Selection(operator) => operator.operate(population, rng),
            Self::Crossover(operator) => operator.operate(population, rng),
            Self::Mutation(operator) => operator.operate(population, rng),
        }

    }
}


#[derive(Debug, Default)]
pub struct SelectionOperator
{

}

impl SelectionOperator
{
    fn find_best_individual(&self, individuals: Vec<Individual>) -> ()
    {

    }
}

impl Operate for SelectionOperator
{
    fn operate(&self, population: &mut Population, rng: &mut impl RngExt) -> ()
    {
        println!("Selecting!");
    }
}


#[derive(Debug, Default)]
pub struct CrossoverOperator
{

}

impl CrossoverOperator
{
    fn mix(&self) -> ()
    {

    }
}

impl Operate for CrossoverOperator
{
    fn operate(&self, population: &mut Population, rng: &mut impl RngExt) -> ()
    {
        println!("Crossing!");
    }
}


#[derive(Debug, Default)]
pub struct MutationOperator 
{

}

impl MutationOperator
{
    fn mutate(&self, individual: &mut Individual, rng: &mut impl RngExt) -> ()
    {
        individual.right_handed = rng.random();
        individual.height += rng.sample::<f64, StandardNormal>(StandardNormal);
        // individual.eye_color = rng.random();

        // pub right_handed: bool,
        // pub height: f64,
        // pub eye_color: EyeColor,
    }
}

impl Operate for MutationOperator
{
    fn operate(&self, population: &mut Population, rng: &mut impl RngExt) -> ()
    {
        for (_, individual) in &mut population.members
        {
            self.mutate(individual, rng);
        }
    }
}


