use dyn_clone::DynClone;
use std::{any::Any, fmt::Debug};

use crate::bo::Paycheck;

pub trait Affiliation: DynClone + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_deductions(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(Affiliation);
