
use std::collections::HashMap;
use uuid::Uuid;
use rand::RngExt;

use crate::individual::Individual;



#[derive(Debug, Clone, Default)]
pub struct Population
{
    pub members: HashMap<Uuid, Individual>,
}


impl Population
{
    pub fn new(size: usize, rng: &mut impl RngExt) -> Self
    {
        Self {
            members: Self::generate_random_members(size, rng),
        }
    }

    fn generate_random_members(size: usize, rng: &mut impl RngExt) -> HashMap<Uuid, Individual>
    {
        // (0..size)
        //     .map(|_| { Individual::new_random(rng) })
        //     .map(|individual| (individual.id, individual))
        //     .collect()

        (0..size)
            .map(
                |_| 
                { 
                    let individual = Individual::new_random(rng);
                    (individual.id, individual)
                }
            )
            .collect()
    }

    pub fn add_member(&mut self, individual: Individual) -> ()
    {
        // NOTE: If the new individual's ID already exists in the HashMap then it will overwrite the
        // old individual with the new individual
        self.members.insert(individual.id, individual);
    }

    pub fn remove_member(&mut self, member_id: Uuid) -> ()
    {
        self.members.remove(&member_id);
    }

    pub fn size(&self) -> usize
    {
        self.members.len()
    }
}



