
use crate::individual::Individual;
use crate::experiment::Population;






pub trait Operate
{
    fn operate(&self, population: &Population, seed: u64) -> ();
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
    fn operate(&self, population: &Population, seed: u64) -> ()
    {
        match self
        {
            Self::Selection(operator) => operator.operate(population, seed),
            Self::Crossover(operator) => operator.operate(population, seed),
            Self::Mutation(operator) => operator.operate(population, seed),
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
    fn operate(&self, population: &Population, seed: u64) -> ()
    {

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
    fn operate(&self, population: &Population, seed: u64) -> ()
    {

    }
}


#[derive(Debug, Default)]
pub struct MutationOperator 
{

}

impl MutationOperator
{
    fn mutate(&self, individual: Individual) -> ()
    {

    }
}

impl Operate for MutationOperator
{
    fn operate(&self, population: &Population, seed: u64) -> ()
    {

    }
}


