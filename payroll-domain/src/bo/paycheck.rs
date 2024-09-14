use chrono::NaiveDate;
use std::{fmt::Debug, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq)]
pub struct Paycheck {
    period: RangeInclusive<NaiveDate>,
    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}
impl Paycheck {
    pub fn new(period: RangeInclusive<NaiveDate>) -> Self {
        Self {
            period,
            gross_pay: 0.0,
            deductions: 0.0,
            net_pay: 0.0,
        }
    }
    pub fn get_period(&self) -> RangeInclusive<NaiveDate> {
        self.period.clone()
    }
    pub fn set_gross_pay(&mut self, gross_pay: f32) {
        self.gross_pay = gross_pay;
    }
    pub fn set_deductions(&mut self, deductions: f32) {
        self.deductions = deductions;
    }
    pub fn get_net_pay(&self) -> f32 {
        self.net_pay
    }
    pub fn set_net_pay(&mut self, net_pay: f32) {
        self.net_pay = net_pay;
    }
}
