
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use rand::RngExt;
use rand::prelude::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};



#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
#[derive(EnumIter)]
pub enum EyeColor
{
    Hazel,
    #[default]
    Brown,
    Green,
    Amber,
    Blue,
    Gray,
    Heterochromia,
}


#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct Individual
{
    pub id: Uuid,

    // Add your individual's attributes (genes) here
    pub right_handed: bool,
    pub height: f64,
    pub eye_color: EyeColor,
}

impl Individual
{
    pub fn new(
        right_handed: bool, 
        height: f64, 
        eye_color: EyeColor
    ) -> Self
    {
        Self {
            id: Uuid::new_v4(),
            right_handed: right_handed,
            height: height,
            eye_color: eye_color,
        }
    }

    pub fn new_random(rng: &mut impl RngExt) -> Self
    {
        // Generate the random attributes
        let random_right_handed: bool = rng.random();
        let random_height: f64 = rng.random();
        // let random_height: f64 = rng.random_range(0.0..100.0);
        let random_eye_color = EyeColor::iter()
            .choose(rng)
            .expect("Enum: EyeColor should have variants");

        Self::new(
            random_right_handed,
            random_height,
            random_eye_color,
        )
    }
}



