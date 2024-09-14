use chrono::NaiveDate;
use std::{cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

use crate::bo::Paycheck;
use crate::interface::{Affiliation, PaymentClassification, PaymentMethod, PaymentSchedule};
use crate::types::EmployeeId;

#[derive(Debug, Clone)]
pub struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,

    classification: Rc<RefCell<dyn PaymentClassification>>,
    schedule: Rc<RefCell<dyn PaymentSchedule>>,
    method: Rc<RefCell<dyn PaymentMethod>>,
    affiliation: Rc<RefCell<dyn Affiliation>>,
}
impl Employee {
    pub fn new(
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
        method: Rc<RefCell<dyn PaymentMethod>>,
        affiliation: Rc<RefCell<dyn Affiliation>>,
    ) -> Self {
        Self {
            emp_id,
            name: name.to_string(),
            address: address.to_string(),
            classification,
            schedule,
            method,
            affiliation,
        }
    }
    pub fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }
    pub fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.classification.clone()
    }
    pub fn set_classification(&mut self, classification: Rc<RefCell<dyn PaymentClassification>>) {
        self.classification = classification;
    }
    pub fn set_schedule(&mut self, schedule: Rc<RefCell<dyn PaymentSchedule>>) {
        self.schedule = schedule;
    }
    pub fn set_method(&mut self, method: Rc<RefCell<dyn PaymentMethod>>) {
        self.method = method;
    }
    pub fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        self.affiliation.clone()
    }
    pub fn set_affiliation(&mut self, affiliation: Rc<RefCell<dyn Affiliation>>) {
        self.affiliation = affiliation;
    }
    pub fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.schedule.borrow().is_pay_date(date)
    }
    pub fn get_pay_period(&self, payday: NaiveDate) -> RangeInclusive<NaiveDate> {
        self.schedule.borrow().calculate_period(payday)
    }
    pub fn payday(&self, pc: &mut Paycheck) {
        let gross_pay = self.classification.borrow().calculate_pay(pc);
        let deductions = self.affiliation.borrow().calculate_deductions(pc);
        let net_pay = gross_pay - deductions;
        pc.set_gross_pay(gross_pay);
        pc.set_deductions(deductions);
        pc.set_net_pay(net_pay);
        self.method.borrow().pay(pc);
    }
}
