use chrono::NaiveDate;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

mod payroll_domain {
    use chrono::NaiveDate;
    use dyn_clone::DynClone;
    use std::{any::Any, cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

    pub type EmployeeId = u32;
    pub type MemberId = u32;

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
        pub fn set_classification(
            &mut self,
            classification: Rc<RefCell<dyn PaymentClassification>>,
        ) {
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

    pub trait PaymentClassification: DynClone + Debug {
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn calculate_pay(&self, pc: &Paycheck) -> f32;
    }
    dyn_clone::clone_trait_object!(PaymentClassification);

    pub trait PaymentSchedule: DynClone + Debug {
        fn is_pay_date(&self, date: NaiveDate) -> bool;
        fn calculate_period(&self, payday: NaiveDate) -> RangeInclusive<NaiveDate>;
    }
    dyn_clone::clone_trait_object!(PaymentSchedule);

    pub trait PaymentMethod: DynClone + Debug {
        fn pay(&self, pc: &Paycheck);
    }
    dyn_clone::clone_trait_object!(PaymentMethod);

    pub trait Affiliation: DynClone + Debug {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn calculate_deductions(&self, pc: &Paycheck) -> f32;
    }
    dyn_clone::clone_trait_object!(Affiliation);
}
use payroll_domain::*;

mod payroll_impl {
    use chrono::{Datelike, Days, NaiveDate, Weekday};
    use std::{any::Any, ops::RangeInclusive};

    use crate::payroll_domain::{
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
                    let calc_pay_for_sales_receipt =
                        |sr: &SalesReceipt| sr.amount * commission_rate;
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
                PaymentScheduleImpl::Weekly => {
                    payday.checked_sub_days(Days::new(6)).unwrap()..=payday
                }
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
}
use payroll_impl::*;

mod dao {
    use thiserror::Error;

    use crate::payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum DaoError {
        #[error("insert error: {0}")]
        InsertError(String),
        #[error("delete error: {0}")]
        DeleteError(String),
        #[error("fetch error: {0}")]
        FetchError(String),
        #[error("update error: {0}")]
        UpdateError(String),
    }

    pub trait PayrollDao<Ctx> {
        fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
        fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
        fn fetch(&self, emp_id: EmployeeId)
            -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
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
        fn find_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
        fn record_paycheck(
            &self,
            emp_id: EmployeeId,
            pc: Paycheck,
        ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    }

    pub trait HavePayrollDao<Ctx> {
        fn dao(&self) -> &impl PayrollDao<Ctx>;
    }
}
use dao::*;

mod mock_db {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use crate::dao::{DaoError, PayrollDao};
    use crate::payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

    #[derive(Debug, Clone)]
    pub struct MockDb {
        employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
        union_members: Rc<RefCell<HashMap<MemberId, EmployeeId>>>,
        paychecks: Rc<RefCell<HashMap<EmployeeId, Vec<Paycheck>>>>,
    }
    impl MockDb {
        pub fn new() -> Self {
            Self {
                employees: Rc::new(RefCell::new(HashMap::new())),
                union_members: Rc::new(RefCell::new(HashMap::new())),
                paychecks: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }
    impl PayrollDao<()> for MockDb {
        fn insert(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |_| {
                let emp_id = emp.get_emp_id();

                if self.employees.borrow().contains_key(&emp_id) {
                    return Err(DaoError::InsertError(format!(
                        "emp_id={} already exists",
                        emp_id
                    )));
                }
                self.employees.borrow_mut().insert(emp_id, emp);
                Ok(emp_id)
            })
        }
        fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<(), Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_| {
                if self.employees.borrow_mut().remove(&emp_id).is_none() {
                    return Err(DaoError::DeleteError(format!(
                        "emp_id={} not found",
                        emp_id
                    )));
                }
                Ok(())
            })
        }
        fn fetch(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<(), Item = Employee, Err = DaoError> {
            tx_rs::with_tx(move |_| match self.employees.borrow().get(&emp_id) {
                Some(emp) => Ok(emp.clone()),
                None => Err(DaoError::FetchError(format!("emp_id={} not found", emp_id))),
            })
        }
        fn update(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_| {
                let emp_id = emp.get_emp_id();

                if !self.employees.borrow().contains_key(&emp_id) {
                    return Err(DaoError::UpdateError(format!(
                        "emp_id={} not found",
                        emp_id
                    )));
                }
                self.employees.borrow_mut().insert(emp_id, emp);
                Ok(())
            })
        }
        fn fetch_all(&self) -> impl tx_rs::Tx<(), Item = Vec<Employee>, Err = DaoError> {
            tx_rs::with_tx(move |_| Ok(self.employees.borrow().values().cloned().collect()))
        }

        fn add_union_member(
            &self,
            member_id: MemberId,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<(), Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_| {
                if self.union_members.borrow().contains_key(&member_id) {
                    return Err(DaoError::InsertError(format!(
                        "member_id={} already exists",
                        member_id
                    )));
                }
                if self.union_members.borrow().values().any(|&v| v == emp_id) {
                    return Err(DaoError::InsertError(format!(
                        "emp_id={} already exists",
                        emp_id
                    )));
                }
                self.union_members.borrow_mut().insert(member_id, emp_id);
                Ok(())
            })
        }
        fn remove_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<(), Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_| {
                if self.union_members.borrow_mut().remove(&member_id).is_none() {
                    return Err(DaoError::DeleteError(format!(
                        "member_id={} not found",
                        member_id
                    )));
                }
                Ok(())
            })
        }
        fn find_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |_| {
                self.union_members
                    .borrow()
                    .get(&member_id)
                    .map(|&v| v)
                    .ok_or(DaoError::FetchError(format!("member_id: {}", member_id)))
            })
        }
        fn record_paycheck(
            &self,
            emp_id: EmployeeId,
            pc: Paycheck,
        ) -> impl tx_rs::Tx<(), Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_| {
                self.paychecks
                    .borrow_mut()
                    .entry(emp_id)
                    .or_insert(vec![])
                    .push(pc);
                Ok(())
            })
        }
    }
}
use mock_db::*;

#[derive(Error, Debug, Clone, PartialEq)]
enum UsecaseError {
    #[error("register employee failed: {0}")]
    RegisterEmployeeFailed(DaoError),
    #[error("unregister employee failed: {0}")]
    UnregisterEmployeeFailed(DaoError),
    #[error("employee not found: {0}")]
    NotFound(DaoError),
    #[error("can't get all employees: {0}")]
    GetAllFailed(DaoError),
    #[error("unexpected payment classification: {0}")]
    UnexpectedPaymentClassification(String),
    #[error("update employee failed: {0}")]
    UpdateEmployeeFailed(DaoError),
    #[error("unexpected affiliation: {0}")]
    UnexpectedAffiliation(String),
    #[error("add union member failed: {0}")]
    AddUnionMemberFailed(DaoError),
    #[error("remove union member failed: {0}")]
    RemoveUnionMemberFailed(DaoError),
}

trait AddEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        let emp = Employee::new(
            emp_id,
            name,
            address,
            classification,
            schedule,
            Rc::new(RefCell::new(PaymentMethodImpl::Hold)),
            Rc::new(RefCell::new(AffiliationImpl::Unaffiliated)),
        );
        self.dao()
            .insert(emp)
            .map_err(UsecaseError::RegisterEmployeeFailed)
    }
}
// blanket implementation
impl<T, Ctx> AddEmployeeTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait AddSalaryEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTx::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(PaymentClassificationImpl::Salaried { salary })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Monthly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddSalaryEmployeeTx<Ctx> for T where T: AddEmployeeTx<Ctx> {}

trait AddHourlyEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTx::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(PaymentClassificationImpl::Hourly {
                hourly_rate,
                timecards: vec![],
            })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Weekly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddHourlyEmployeeTx<Ctx> for T where T: AddEmployeeTx<Ctx> {}

trait AddCommissionedEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        AddEmployeeTx::execute(
            self,
            emp_id,
            name,
            address,
            Rc::new(RefCell::new(PaymentClassificationImpl::Commissioned {
                salary,
                commission_rate,
                sales_receipts: vec![],
            })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Biweekly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> AddCommissionedEmployeeTx<Ctx> for T where T: AddEmployeeTx<Ctx> {}

trait DeleteEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .delete(emp_id)
            .map_err(UsecaseError::UnregisterEmployeeFailed)
    }
}
// blanket implementation
impl<T, Ctx> DeleteEmployeeTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ChangeEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        f: F,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), UsecaseError>,
    {
        tx_rs::with_tx(move |ctx| {
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            f(ctx, &mut emp)?;
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ChangeEmployeeNameTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        name: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_name(name);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeNameTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}

trait ChangeEmployeeAddressTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        address: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_address(address);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeAddressTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}

trait ChangeEmployeePaymentClassificationTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_classification(classification);
            emp.set_schedule(schedule);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeePaymentClassificationTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}

trait ChangeEmployeeSalariedTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        salary: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentClassificationTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentClassificationImpl::Salaried { salary })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Monthly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeSalariedTx<Ctx> for T where T: ChangeEmployeePaymentClassificationTx<Ctx> {}

trait ChangeEmployeeHourlyTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        hourly_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentClassificationTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentClassificationImpl::Hourly {
                hourly_rate,
                timecards: vec![],
            })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Weekly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeHourlyTx<Ctx> for T where T: ChangeEmployeePaymentClassificationTx<Ctx> {}

trait ChangeEmployeeCommissionedTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentClassificationTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentClassificationImpl::Commissioned {
                salary,
                commission_rate,
                sales_receipts: vec![],
            })),
            Rc::new(RefCell::new(PaymentScheduleImpl::Biweekly)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeCommissionedTx<Ctx> for T where
    T: ChangeEmployeePaymentClassificationTx<Ctx>
{
}

trait ChangeEmployeePaymentMethodTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        method: Rc<RefCell<dyn PaymentMethod>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeeTx::execute(self, emp_id, |_, emp| {
            emp.set_method(method);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeePaymentMethodTx<Ctx> for T where T: ChangeEmployeeTx<Ctx> {}

trait ChangeEmployeeHoldTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentMethodTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentMethodImpl::Hold)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeHoldTx<Ctx> for T where T: ChangeEmployeePaymentMethodTx<Ctx> {}

trait ChangeEmployeeDirectTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        bank: &str,
        account: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentMethodTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentMethodImpl::Direct {
                bank: bank.to_string(),
                account: account.to_string(),
            })),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeDirectTx<Ctx> for T where T: ChangeEmployeePaymentMethodTx<Ctx> {}

trait ChangeEmployeeMailTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        address: &str,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeEmployeePaymentMethodTx::execute(
            self,
            emp_id,
            Rc::new(RefCell::new(PaymentMethodImpl::Mail {
                address: address.to_string(),
            })),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeEmployeeMailTx<Ctx> for T where T: ChangeEmployeePaymentMethodTx<Ctx> {}

trait TimeCardTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<PaymentClassificationImpl>()
                .ok_or(UsecaseError::UnexpectedPaymentClassification(format!(
                    "expected hourly emp_id: {}",
                    emp_id
                )))?
                .add_timecard(TimeCard::new(date, hours));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> TimeCardTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait SalesReceiptTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<PaymentClassificationImpl>()
                .ok_or(UsecaseError::UnexpectedPaymentClassification(format!(
                    "expected commissioned emp_id: {}",
                    emp_id
                )))?
                .add_sales_receipt(SalesReceipt::new(date, amount));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> SalesReceiptTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ChangeAffiliationTx<Ctx>: ChangeEmployeeTx<Ctx> {
    fn execute<'a, F>(
        &'a self,
        emp_id: EmployeeId,
        record_membership: F,
        affiliation: Rc<RefCell<dyn Affiliation>>,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        F: FnOnce(&mut Ctx, &mut Employee) -> Result<(), UsecaseError>,
        Ctx: 'a,
    {
        ChangeEmployeeTx::<Ctx>::execute(self, emp_id, |ctx, emp| {
            record_membership(ctx, emp)?;
            emp.set_affiliation(affiliation);
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> ChangeAffiliationTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ChangeUnionMemberTx<Ctx>: ChangeAffiliationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTx::execute(
            self,
            emp_id,
            move |ctx, _| {
                self.dao()
                    .add_union_member(member_id, emp_id)
                    .run(ctx)
                    .map_err(UsecaseError::AddUnionMemberFailed)
            },
            Rc::new(RefCell::new(AffiliationImpl::Union {
                member_id,
                dues,
                service_charges: vec![],
            })),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnionMemberTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ChangeUnaffiliatedTx<Ctx>: ChangeAffiliationTx<Ctx> {
    fn execute<'a>(
        &'a self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        ChangeAffiliationTx::execute(
            self,
            emp_id,
            move |ctx, emp| {
                let member_id = emp
                    .get_affiliation()
                    .borrow()
                    .as_any()
                    .downcast_ref::<AffiliationImpl>()
                    .map_or(
                        Err(UsecaseError::UnexpectedAffiliation(format!(
                            "expected unaffiliated emp_id: {}",
                            emp_id
                        ))),
                        |a| Ok(a.get_member_id()),
                    )?;
                self.dao()
                    .remove_union_member(member_id)
                    .run(ctx)
                    .map_err(UsecaseError::RemoveUnionMemberFailed)
            },
            Rc::new(RefCell::new(AffiliationImpl::Unaffiliated)),
        )
    }
}
// blanket implementation
impl<T, Ctx> ChangeUnaffiliatedTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait ServiceChargeTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self
                .dao()
                .find_union_member(member_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::NotFound)?;
            emp.get_affiliation()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<AffiliationImpl>()
                .ok_or(UsecaseError::UnexpectedAffiliation(format!(
                    "expected union emp_id: {}",
                    emp_id
                )))?
                .add_service_charge(ServiceCharge::new(date, amount));
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::UpdateEmployeeFailed)
        })
    }
}
// blanket implementation
impl<T, Ctx> ServiceChargeTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait PaydayTx<Ctx>: HavePayrollDao<Ctx> {
    fn execute<'a>(
        &'a self,
        pay_date: NaiveDate,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emps = self
                .dao()
                .fetch_all()
                .run(ctx)
                .map_err(UsecaseError::GetAllFailed)?;
            for emp in emps {
                if emp.is_pay_date(pay_date) {
                    let period = emp.get_pay_period(pay_date);
                    let mut pc = Paycheck::new(period);
                    emp.payday(&mut pc);
                    self.dao()
                        .record_paycheck(emp.get_emp_id(), pc)
                        .run(ctx)
                        .map_err(UsecaseError::UpdateEmployeeFailed)?;
                }
            }
            Ok(())
        })
    }
}
// blanket implementation
impl<T, Ctx> PaydayTx<Ctx> for T where T: HavePayrollDao<Ctx> {}

trait Transaction<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<(), UsecaseError>;
}

struct AddSalaryEmployeeTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f32,
}
impl HavePayrollDao<()> for AddSalaryEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for AddSalaryEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddSalaryEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

struct AddHourlyEmployeeTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f32,
}
impl HavePayrollDao<()> for AddHourlyEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for AddHourlyEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddHourlyEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.hourly_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

struct AddCommissionedEmployeeTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f32,
    commission_rate: f32,
}
impl HavePayrollDao<()> for AddCommissionedEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for AddCommissionedEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        AddCommissionedEmployeeTx::execute(
            self,
            self.emp_id,
            &self.name,
            &self.address,
            self.salary.clone(),
            self.commission_rate.clone(),
        )
        .map(|_| ())
        .run(ctx)
    }
}

struct ChangeEmployeeNameTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    name: String,
}
impl HavePayrollDao<()> for ChangeEmployeeNameTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeNameTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeNameTx::execute(self, self.emp_id, &self.name)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeAddressTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeAddressTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeAddressTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeAddressTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeSalariedTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    salary: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeSalariedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeSalariedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeSalariedTx::execute(self, self.emp_id, self.salary)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeHourlyTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    hourly_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeHourlyTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeHourlyTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHourlyTx::execute(self, self.emp_id, self.hourly_rate)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeCommissionedTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    salary: f32,
    commission_rate: f32,
}
impl HavePayrollDao<()> for ChangeEmployeeCommissionedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeCommissionedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeCommissionedTx::execute(self, self.emp_id, self.salary, self.commission_rate)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeHoldTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeEmployeeHoldTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeHoldTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeHoldTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeMailTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    address: String,
}
impl HavePayrollDao<()> for ChangeEmployeeMailTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeMailTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeMailTx::execute(self, self.emp_id, &self.address)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeEmployeeDirectTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    bank: String,
    account: String,
}
impl HavePayrollDao<()> for ChangeEmployeeDirectTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeEmployeeDirectTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeEmployeeDirectTx::execute(self, self.emp_id, &self.bank, &self.account)
            .map(|_| ())
            .run(ctx)
    }
}

struct TimeCardTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    date: NaiveDate,
    hours: f32,
}
impl HavePayrollDao<()> for TimeCardTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for TimeCardTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        TimeCardTx::execute(self, self.emp_id, self.date, self.hours)
            .map(|_| ())
            .run(ctx)
    }
}

struct SalesReceiptTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    date: NaiveDate,
    amount: f32,
}
impl HavePayrollDao<()> for SalesReceiptTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for SalesReceiptTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        SalesReceiptTx::execute(self, self.emp_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeUnionMemberTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
    member_id: MemberId,
    dues: f32,
}
impl HavePayrollDao<()> for ChangeUnionMemberTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeUnionMemberTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnionMemberTx::execute(self, self.emp_id, self.member_id, self.dues)
            .map(|_| ())
            .run(ctx)
    }
}

struct ChangeUnaffiliatedTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
}
impl HavePayrollDao<()> for ChangeUnaffiliatedTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ChangeUnaffiliatedTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ChangeUnaffiliatedTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

struct ServiceChargeTxImpl {
    dao: MockDb,

    member_id: MemberId,
    date: NaiveDate,
    amount: f32,
}
impl HavePayrollDao<()> for ServiceChargeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for ServiceChargeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        ServiceChargeTx::execute(self, self.member_id, self.date, self.amount)
            .map(|_| ())
            .run(ctx)
    }
}

struct DeleteEmployeeTxImpl {
    dao: MockDb,

    emp_id: EmployeeId,
}
impl HavePayrollDao<()> for DeleteEmployeeTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for DeleteEmployeeTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        DeleteEmployeeTx::execute(self, self.emp_id)
            .map(|_| ())
            .run(ctx)
    }
}

struct PaydayTxImpl {
    dao: MockDb,

    date: NaiveDate,
}
impl HavePayrollDao<()> for PaydayTxImpl {
    fn dao(&self) -> &impl PayrollDao<()> {
        &self.dao
    }
}
impl Transaction<()> for PaydayTxImpl {
    fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
        PaydayTx::execute(self, self.date).map(|_| ()).run(ctx)
    }
}

fn main() {
    let db = MockDb::new();

    let tx: Box<dyn Transaction<()>> = Box::new(AddSalaryEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 1,
        name: "Bob".to_string(),
        address: "Home".to_string(),
        salary: 1020.75,
    });
    tx.execute(&mut ()).expect("add salary employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(AddHourlyEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 2,
        name: "Alice".to_string(),
        address: "Home".to_string(),
        hourly_rate: 10.5,
    });
    tx.execute(&mut ()).expect("add hourly employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(AddCommissionedEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 3,
        name: "Charlie".to_string(),
        address: "Home".to_string(),
        salary: 420.0,
        commission_rate: 0.25,
    });
    tx.execute(&mut ()).expect("add commissioned employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(AddSalaryEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 4,
        name: "Daby".to_string(),
        address: "Home".to_string(),
        salary: 1020.50,
    });
    tx.execute(&mut ()).expect("add salary employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeNameTxImpl {
        dao: db.clone(),
        emp_id: 3,
        name: "Chris".to_string(),
    });
    tx.execute(&mut ()).expect("change name");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeAddressTxImpl {
        dao: db.clone(),
        emp_id: 3,
        address: "Office".to_string(),
    });
    tx.execute(&mut ()).expect("change address");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeHourlyTxImpl {
        dao: db.clone(),
        emp_id: 4,
        hourly_rate: 12.5,
    });
    tx.execute(&mut ()).expect("change employee to hourly");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeCommissionedTxImpl {
        dao: db.clone(),
        emp_id: 4,
        salary: 420.0,
        commission_rate: 0.30,
    });
    tx.execute(&mut ())
        .expect("change employee to commissioned");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeSalariedTxImpl {
        dao: db.clone(),
        emp_id: 4,
        salary: 1100.25,
    });
    tx.execute(&mut ()).expect("change employee to salaried");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeMailTxImpl {
        dao: db.clone(),
        emp_id: 4,
        address: "daby@example.com".to_string(),
    });
    tx.execute(&mut ()).expect("change mail");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeDirectTxImpl {
        dao: db.clone(),
        emp_id: 3,
        bank: "mufg".to_string(),
        account: "1234567".to_string(),
    });
    tx.execute(&mut ()).expect("change direct");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeMailTxImpl {
        dao: db.clone(),
        emp_id: 2,
        address: "bob@example.com".to_string(),
    });
    tx.execute(&mut ()).expect("change mail");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeEmployeeHoldTxImpl {
        dao: db.clone(),
        emp_id: 4,
    });
    tx.execute(&mut ()).expect("change hold");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(TimeCardTxImpl {
        dao: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 9, 11).unwrap(),
        hours: 8.0,
    });
    tx.execute(&mut ()).expect("add time card");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(TimeCardTxImpl {
        dao: db.clone(),
        emp_id: 2,
        date: NaiveDate::from_ymd_opt(2024, 9, 12).unwrap(),
        hours: 8.5,
    });
    tx.execute(&mut ()).expect("add time card");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(SalesReceiptTxImpl {
        dao: db.clone(),
        emp_id: 3,
        date: NaiveDate::from_ymd_opt(2024, 9, 17).unwrap(),
        amount: 12300.0,
    });
    tx.execute(&mut ()).expect("add sales receipt");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(SalesReceiptTxImpl {
        dao: db.clone(),
        emp_id: 3,
        date: NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(),
        amount: 3210.0,
    });
    tx.execute(&mut ()).expect("add sales receipt");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeUnionMemberTxImpl {
        dao: db.clone(),
        emp_id: 1,
        member_id: 7124,
        dues: 9.75,
    });
    tx.execute(&mut ()).expect("change union member");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ServiceChargeTxImpl {
        dao: db.clone(),
        member_id: 7124,
        date: NaiveDate::from_ymd_opt(2024, 9, 12).unwrap(),
        amount: 371.50,
    });
    tx.execute(&mut ()).expect("add service charge");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ServiceChargeTxImpl {
        dao: db.clone(),
        member_id: 7124,
        date: NaiveDate::from_ymd_opt(2024, 9, 26).unwrap(),
        amount: 123.75,
    });
    tx.execute(&mut ()).expect("add service charge");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(PaydayTxImpl {
        dao: db.clone(),
        date: NaiveDate::from_ymd_opt(2024, 9, 13).unwrap(),
    });
    tx.execute(&mut ()).expect("payday");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(PaydayTxImpl {
        dao: db.clone(),
        date: NaiveDate::from_ymd_opt(2024, 9, 20).unwrap(),
    });
    tx.execute(&mut ()).expect("payday");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(PaydayTxImpl {
        dao: db.clone(),
        date: NaiveDate::from_ymd_opt(2024, 9, 28).unwrap(),
    });
    tx.execute(&mut ()).expect("payday");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(PaydayTxImpl {
        dao: db.clone(),
        date: NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(),
    });
    tx.execute(&mut ()).expect("payday");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(ChangeUnaffiliatedTxImpl {
        dao: db.clone(),
        emp_id: 1,
    });
    tx.execute(&mut ()).expect("change unaffiliated");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(DeleteEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 1,
    });
    tx.execute(&mut ()).expect("delete employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(DeleteEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 2,
    });
    tx.execute(&mut ()).expect("delete employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(DeleteEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 3,
    });
    tx.execute(&mut ()).expect("delete employee");
    println!("{:#?}", db);

    let tx: Box<dyn Transaction<()>> = Box::new(DeleteEmployeeTxImpl {
        dao: db.clone(),
        emp_id: 4,
    });
    tx.execute(&mut ()).expect("delete employee");
    println!("{:#?}", db);
}
