use chrono::NaiveDate;
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCard {
    date: NaiveDate,
    hours: f32,
}
impl TimeCard {
    pub fn new(date: NaiveDate, hours: f32) -> Self {
        Self { date, hours }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SalesReceipt {
    date: NaiveDate,
    amount: f32,
}
impl SalesReceipt {
    pub fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentClassificationImpl {
    Salaried {
        salary: f32,
    },
    Hourly {
        hourly_rate: f32,
        timecards: Vec<TimeCard>,
    },
    Commissioned {
        salary: f32,
        commission_rate: f32,
        sales_receipts: Vec<SalesReceipt>,
    },
}
impl PaymentClassificationImpl {
    pub fn add_timecard(&mut self, tc: TimeCard) {
        match self {
            PaymentClassificationImpl::Hourly { timecards, .. } => {
                timecards.push(tc);
            }
            _ => {
                panic!("Timecard is not applicable for this classification");
            }
        }
    }
    pub fn add_sales_receipt(&mut self, sr: SalesReceipt) {
        match self {
            PaymentClassificationImpl::Commissioned { sales_receipts, .. } => {
                sales_receipts.push(sr);
            }
            _ => {
                panic!("Sales receipt is not applicable for this classification");
            }
        }
    }
}
impl PaymentClassification for PaymentClassificationImpl {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
        match self {
            PaymentClassificationImpl::Salaried { salary } => *salary,
            PaymentClassificationImpl::Hourly {
                hourly_rate,
                timecards,
            } => {
                let calc_pay_for_timecard = |tc: &TimeCard| {
                    let hours = tc.hours;
                    let overtime = (hours - 8.0).max(0.0);
                    let straight_time = hours - overtime;
                    straight_time * hourly_rate + overtime * hourly_rate * 1.5
                };
                let period = pc.get_period();
                let mut total_pay = 0.0;
                for tc in timecards {
                    if period.contains(&tc.date) {
                        total_pay += calc_pay_for_timecard(tc);
                    }
                }
                total_pay
            }
            PaymentClassificationImpl::Commissioned {
                salary,
                commission_rate,
                sales_receipts,
            } => {
                let calc_pay_for_sales_receipt = |sr: &SalesReceipt| sr.amount * commission_rate;
                let period = pc.get_period();
                let mut total_pay = *salary;
                for sr in sales_receipts {
                    if period.contains(&sr.date) {
                        total_pay += calc_pay_for_sales_receipt(sr);
                    }
                }
                total_pay
            }
        }
    }
}
