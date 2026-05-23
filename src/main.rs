
use experiment::{ExperimentBuilder, ExperimentExitCriteria};
use operators::Operator;

mod experiment;
mod individual;
mod operators;



fn main() 
{
    let mut experiment = ExperimentBuilder::new()
        .exit_criteria(ExperimentExitCriteria::IterationCount(1_000_000))
        .seed(6240)
        .operator(Operator::Mutation(Default::default()))
        .build();
    // println!("{:#?}", experiment);


    let experiment_result = experiment.run().expect("Experiment failed while running");
    println!("{:#?}", experiment);
    println!("{:#?}", experiment_result);
}




