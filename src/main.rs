use chrono::{Datelike, Days, NaiveDate, Weekday};
use dyn_clone::DynClone;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use thiserror::Error;
use tx_rs::Tx;

type EmployeeId = u32;
type MemberId = u32;

#[derive(Debug, Clone)]
struct Employee {
    emp_id: EmployeeId,
    name: String,
    address: String,

    classification: Box<dyn PaymentClassification>,
    schedule: Box<dyn PaymentSchedule>,
    method: Box<dyn PaymentMethod>,
}

#[derive(Debug, Clone, PartialEq)]
struct Paycheck {
    period: RangeInclusive<NaiveDate>,
    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}

trait PaymentClassification: DynClone + Debug {
    fn calculate_pay(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(PaymentClassification);

#[derive(Debug, Clone, PartialEq)]
struct TimeCard {
    date: NaiveDate,
    hours: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct SalesReceipt {
    date: NaiveDate,
    amount: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum PaymentClassificationImpl {
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
impl PaymentClassification for PaymentClassificationImpl {
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
                let period = &pc.period;
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
                let period = &pc.period;
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

trait PaymentSchedule: DynClone + Debug {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn calculate_period(&self, payday: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);

#[derive(Debug, Clone, PartialEq)]
enum PaymentScheduleImpl {
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

trait PaymentMethod: DynClone + Debug {
    fn pay(&self, pc: &Paycheck);
}
dyn_clone::clone_trait_object!(PaymentMethod);

#[derive(Debug, Clone, PartialEq)]
enum PaymentMethodImpl {
    Hold,
    Mail { address: String },
    Direct { bank: String, account: String },
}
impl PaymentMethod for PaymentMethodImpl {
    fn pay(&self, pc: &Paycheck) {
        match self {
            PaymentMethodImpl::Hold => {
                println!("Hold the check");
            }
            PaymentMethodImpl::Mail { address } => {
                println!("Send check to {} by Mail", address);
            }
            PaymentMethodImpl::Direct { bank, account } => {
                println!("Direct deposit ${} to {} at {}", pc.net_pay, account, bank);
            }
        }
    }
}

trait Affiliation: DynClone + Debug {
    fn calculate_deductions(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(Affiliation);

#[derive(Debug, Clone, PartialEq)]
struct ServiceCharge {
    date: NaiveDate,
    amount: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum AffiliationImpl {
    NoAffiliation,
    Union {
        member_id: u32,
        dues: f32,
        service_charges: Vec<ServiceCharge>,
    },
}
impl Affiliation for AffiliationImpl {
    fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
        match self {
            AffiliationImpl::NoAffiliation => 0.0,
            AffiliationImpl::Union {
                dues,
                service_charges,
                ..
            } => {
                let mut total_deductions = 0.0;
                let period = &pc.period;
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

#[derive(Error, Debug, Clone, PartialEq, Eq)]
enum DaoError {
    #[error("dummy")]
    Dummy,
}

trait PayrollDao<Ctx> {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
    fn delete(&self, id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn fetch(&self, id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn fetch_all(&self) -> impl tx_rs::Tx<Ctx, Item = Vec<Employee>, Err = DaoError>;
    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn record_paycheck(
        &self,
        emp_id: EmployeeId,
        pc: Paycheck,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
}

trait HavePayrollDao<Ctx> {
    fn dao(&self) -> &impl PayrollDao<Ctx>;
}

fn main() {
    println!("Hello, world!");
}
