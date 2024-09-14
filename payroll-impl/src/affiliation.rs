use chrono::{Datelike, NaiveDate, Weekday};
use std::any::Any;

use payroll_domain::{Affiliation, MemberId, Paycheck};

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceCharge {
    date: NaiveDate,
    amount: f32,
}
impl ServiceCharge {
    pub fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AffiliationImpl {
    Unaffiliated,
    Union {
        member_id: MemberId,
        dues: f32,
        service_charges: Vec<ServiceCharge>,
    },
}
impl AffiliationImpl {
    pub fn add_service_charge(&mut self, sc: ServiceCharge) {
        match self {
            AffiliationImpl::Unaffiliated => {
                panic!("Service charge is not applicable for unaffiliated");
            }
            AffiliationImpl::Union {
                service_charges, ..
            } => {
                service_charges.push(sc);
            }
        }
    }
    pub fn get_member_id(&self) -> MemberId {
        match self {
            AffiliationImpl::Unaffiliated => panic!("Unaffiliated has no member id"),
            AffiliationImpl::Union { member_id, .. } => *member_id,
        }
    }
}
impl Affiliation for AffiliationImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
        match self {
            AffiliationImpl::Unaffiliated => 0.0,
            AffiliationImpl::Union {
                dues,
                service_charges,
                ..
            } => {
                let mut total_deductions = 0.0;
                let period = pc.get_period();
                for d in period.start().iter_days() {
                    if d > *period.end() {
                        break;
                    }
                    if d.weekday() == Weekday::Fri {
                        total_deductions += dues;
                    }
                }
                for sc in service_charges {
                    if period.contains(&sc.date) {
                        total_deductions += sc.amount;
                    }
                }
                total_deductions
            }
        }
    }
}
