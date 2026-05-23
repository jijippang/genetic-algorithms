
use experiment::{ExperimentBuilder, Experiment};

mod experiment;
mod individual;
mod operators;



fn main() 
{
    let mut experiment = ExperimentBuilder::new()
        .seed(2424)
        .build();
    println!("{:#?}", experiment);


    let experiment_result = experiment.run().expect("Experiment failed while running");
    println!("{:#?}", experiment_result);
}




