
use experiment::{ExperimentBuilder, ExperimentExitCriteria};
use operators::Operator;

mod experiment;
mod individual;
mod population;
mod operators;



fn main() 
{
    let mut experiment = ExperimentBuilder::new()
        // .exit_criteria(ExperimentExitCriteria::IterationCount(1_000_000))
        .exit_criteria(ExperimentExitCriteria::DurationSeconds(0.1))
        .seed(98242)
        .operator(Operator::Mutation(Default::default()))
        .build();
    // println!("{:#?}", experiment);


    let experiment_result = experiment.run().expect("Experiment failed while running");
    println!("{:#?}", experiment);
    println!("{:#?}", experiment_result);
}




