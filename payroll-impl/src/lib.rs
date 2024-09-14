use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::{any::Any, ops::RangeInclusive};

use payroll_domain::{
    Affiliation, MemberId, Paycheck, PaymentClassification, PaymentMethod, PaymentSchedule,
};

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

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentScheduleImpl {
    Monthly,
    Weekly,
    Biweekly,
}
impl PaymentSchedule for PaymentScheduleImpl {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        match self {
            PaymentScheduleImpl::Monthly => {
                date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
            }
            PaymentScheduleImpl::Weekly => date.weekday() == Weekday::Fri,
            PaymentScheduleImpl::Biweekly => {
                date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
            }
        }
    }

    fn calculate_period(&self, payday: NaiveDate) -> RangeInclusive<NaiveDate> {
        match self {
            PaymentScheduleImpl::Monthly => payday.with_day(1).unwrap()..=payday,
            PaymentScheduleImpl::Weekly => payday.checked_sub_days(Days::new(6)).unwrap()..=payday,
            PaymentScheduleImpl::Biweekly => {
                payday.checked_sub_days(Days::new(13)).unwrap()..=payday
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodImpl {
    Hold,
    Mail { address: String },
    Direct { bank: String, account: String },
}
impl PaymentMethod for PaymentMethodImpl {
    fn pay(&self, pc: &Paycheck) {
        match self {
            PaymentMethodImpl::Hold => {
                println!("Hold the check: {:#?}", pc);
            }
            PaymentMethodImpl::Mail { address } => {
                println!("Send check to {} by Mail: {:#?}", address, pc);
            }
            PaymentMethodImpl::Direct { bank, account } => {
                println!(
                    "Direct deposit ${} to {} at {}: {:#?}",
                    pc.get_net_pay(),
                    account,
                    bank,
                    pc
                );
            }
        }
    }
}

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
