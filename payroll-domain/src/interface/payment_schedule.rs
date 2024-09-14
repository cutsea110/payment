use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::{fmt::Debug, ops::RangeInclusive};

pub trait PaymentSchedule: DynClone + Debug {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn calculate_period(&self, payday: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);
