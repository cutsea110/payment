use abstract_tx::UsecaseError;
use chrono::NaiveDate;

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

mod abstract_tx {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};
    use thiserror::Error;
    use tx_rs::Tx;

    use crate::dao::{DaoError, HavePayrollDao, PayrollDao};
    use crate::payroll_domain::{
        Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
    };
    use crate::payroll_impl::{AffiliationImpl, PaymentMethodImpl};

    #[derive(Error, Debug, Clone, PartialEq)]
    pub enum UsecaseError {
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

    pub trait AddEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait ChangeEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait ChangeAffiliationTx<Ctx>: ChangeEmployeeTx<Ctx> {
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

    pub trait ChangeEmployeePaymentClassificationTx<Ctx>: ChangeEmployeeTx<Ctx> {
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

    pub trait ChangeEmployeePaymentMethodTx<Ctx>: ChangeEmployeeTx<Ctx> {
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
}

mod tx_impl {
    use chrono::NaiveDate;
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use crate::abstract_tx::{
        AddEmployeeTx, ChangeAffiliationTx, ChangeEmployeePaymentClassificationTx,
        ChangeEmployeePaymentMethodTx, ChangeEmployeeTx, UsecaseError,
    };
    use crate::dao::{HavePayrollDao, PayrollDao};
    use crate::payroll_domain::{EmployeeId, MemberId, Paycheck};
    use crate::payroll_impl::{
        AffiliationImpl, PaymentClassificationImpl, PaymentMethodImpl, PaymentScheduleImpl,
        SalesReceipt, ServiceCharge, TimeCard,
    };

    pub trait AddSalaryEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
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

    pub trait AddHourlyEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
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

    pub trait AddCommissionedEmployeeTx<Ctx>: AddEmployeeTx<Ctx> {
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

    pub trait TimeCardTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait SalesReceiptTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait PaydayTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait DeleteEmployeeTx<Ctx>: HavePayrollDao<Ctx> {
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

    pub trait ChangeEmployeeNameTx<Ctx>: ChangeEmployeeTx<Ctx> {
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

    pub trait ChangeEmployeeAddressTx<Ctx>: ChangeEmployeeTx<Ctx> {
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

    pub trait ChangeEmployeeSalariedTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
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

    pub trait ChangeEmployeeHourlyTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
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

    pub trait ChangeEmployeeCommissionedTx<Ctx>:
        ChangeEmployeePaymentClassificationTx<Ctx>
    {
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

    pub trait ChangeEmployeeHoldTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
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

    pub trait ChangeEmployeeDirectTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
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

    pub trait ChangeEmployeeMailTx<Ctx>: ChangeEmployeePaymentMethodTx<Ctx> {
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

    pub trait ChangeUnionMemberTx<Ctx>: ChangeAffiliationTx<Ctx> {
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

    pub trait ChangeUnaffiliatedTx<Ctx>: ChangeAffiliationTx<Ctx> {
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

    pub trait ServiceChargeTx<Ctx>: HavePayrollDao<Ctx> {
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
}

mod tx_app {
    use crate::abstract_tx::UsecaseError;

    pub trait Transaction<Ctx> {
        fn execute(&self, ctx: &mut Ctx) -> Result<(), UsecaseError>;
    }

    pub trait TransactionSource<Ctx> {
        fn get_transaction(&mut self) -> Option<Box<dyn Transaction<Ctx>>>;
    }

    pub trait TransactionApplication<Ctx> {
        fn tx_source(&self) -> impl TransactionSource<Ctx>;
        fn run(&mut self, ctx: &mut Ctx) -> Result<(), UsecaseError> {
            let mut tx_source = self.tx_source();
            while let Some(tx) = tx_source.get_transaction() {
                let _ = tx.execute(ctx);
            }
            Ok(())
        }
    }
}
use tx_app::*;

mod mock_tx_impl {
    use chrono::NaiveDate;
    use tx_rs::Tx;

    use crate::abstract_tx::UsecaseError;
    use crate::dao::{HavePayrollDao, PayrollDao};
    use crate::mock_db::MockDb;
    use crate::payroll_domain::{EmployeeId, MemberId};
    use crate::tx_app::Transaction;
    use crate::tx_impl::{
        AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalaryEmployeeTx,
        ChangeEmployeeAddressTx, ChangeEmployeeCommissionedTx, ChangeEmployeeDirectTx,
        ChangeEmployeeHoldTx, ChangeEmployeeHourlyTx, ChangeEmployeeMailTx, ChangeEmployeeNameTx,
        ChangeEmployeeSalariedTx, ChangeUnaffiliatedTx, ChangeUnionMemberTx, DeleteEmployeeTx,
        PaydayTx, SalesReceiptTx, ServiceChargeTx, TimeCardTx,
    };

    pub struct AddSalaryEmployeeTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub name: String,
        pub address: String,
        pub salary: f32,
    }
    impl HavePayrollDao<()> for AddSalaryEmployeeTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
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

    pub struct AddHourlyEmployeeTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub name: String,
        pub address: String,
        pub hourly_rate: f32,
    }
    impl HavePayrollDao<()> for AddHourlyEmployeeTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
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

    pub struct AddCommissionedEmployeeTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub name: String,
        pub address: String,
        pub salary: f32,
        pub commission_rate: f32,
    }
    impl HavePayrollDao<()> for AddCommissionedEmployeeTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
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

    pub struct ChangeEmployeeNameTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub name: String,
    }
    impl HavePayrollDao<()> for ChangeEmployeeNameTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeNameTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeNameTx::execute(self, self.emp_id, &self.name)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeAddressTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub address: String,
    }
    impl HavePayrollDao<()> for ChangeEmployeeAddressTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeAddressTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeAddressTx::execute(self, self.emp_id, &self.address)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeSalariedTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub salary: f32,
    }
    impl HavePayrollDao<()> for ChangeEmployeeSalariedTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeSalariedTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeSalariedTx::execute(self, self.emp_id, self.salary)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeHourlyTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub hourly_rate: f32,
    }
    impl HavePayrollDao<()> for ChangeEmployeeHourlyTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeHourlyTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeHourlyTx::execute(self, self.emp_id, self.hourly_rate)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeCommissionedTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub salary: f32,
        pub commission_rate: f32,
    }
    impl HavePayrollDao<()> for ChangeEmployeeCommissionedTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeCommissionedTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeCommissionedTx::execute(
                self,
                self.emp_id,
                self.salary,
                self.commission_rate,
            )
            .map(|_| ())
            .run(ctx)
        }
    }

    pub struct ChangeEmployeeHoldTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
    }
    impl HavePayrollDao<()> for ChangeEmployeeHoldTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeHoldTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeHoldTx::execute(self, self.emp_id)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeMailTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub address: String,
    }
    impl HavePayrollDao<()> for ChangeEmployeeMailTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeMailTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeMailTx::execute(self, self.emp_id, &self.address)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeEmployeeDirectTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub bank: String,
        pub account: String,
    }
    impl HavePayrollDao<()> for ChangeEmployeeDirectTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeEmployeeDirectTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeEmployeeDirectTx::execute(self, self.emp_id, &self.bank, &self.account)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct TimeCardTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub date: NaiveDate,
        pub hours: f32,
    }
    impl HavePayrollDao<()> for TimeCardTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for TimeCardTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            TimeCardTx::execute(self, self.emp_id, self.date, self.hours)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct SalesReceiptTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub date: NaiveDate,
        pub amount: f32,
    }
    impl HavePayrollDao<()> for SalesReceiptTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for SalesReceiptTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            SalesReceiptTx::execute(self, self.emp_id, self.date, self.amount)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeUnionMemberTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
        pub member_id: MemberId,
        pub dues: f32,
    }
    impl HavePayrollDao<()> for ChangeUnionMemberTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeUnionMemberTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeUnionMemberTx::execute(self, self.emp_id, self.member_id, self.dues)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ChangeUnaffiliatedTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
    }
    impl HavePayrollDao<()> for ChangeUnaffiliatedTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ChangeUnaffiliatedTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ChangeUnaffiliatedTx::execute(self, self.emp_id)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct ServiceChargeTxImpl {
        pub db: MockDb,

        pub member_id: MemberId,
        pub date: NaiveDate,
        pub amount: f32,
    }
    impl HavePayrollDao<()> for ServiceChargeTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for ServiceChargeTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            ServiceChargeTx::execute(self, self.member_id, self.date, self.amount)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct DeleteEmployeeTxImpl {
        pub db: MockDb,

        pub emp_id: EmployeeId,
    }
    impl HavePayrollDao<()> for DeleteEmployeeTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for DeleteEmployeeTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            DeleteEmployeeTx::execute(self, self.emp_id)
                .map(|_| ())
                .run(ctx)
        }
    }

    pub struct PaydayTxImpl {
        pub db: MockDb,

        pub pay_date: NaiveDate,
    }
    impl HavePayrollDao<()> for PaydayTxImpl {
        fn dao(&self) -> &impl PayrollDao<()> {
            &self.db
        }
    }
    impl Transaction<()> for PaydayTxImpl {
        fn execute<'a>(&'a self, ctx: &mut ()) -> Result<(), UsecaseError> {
            PaydayTx::execute(self, self.pay_date).map(|_| ()).run(ctx)
        }
    }
}
use mock_tx_impl::*;

mod text_parser_tx_source {
    use parsec_rs::Parser;
    use std::collections::VecDeque;

    use crate::mock_db::MockDb;
    use crate::mock_tx_impl::*;
    use crate::parser::{transactions, Command};
    use crate::tx_app::{Transaction, TransactionSource};

    pub struct TextParserTransactionSource {
        txs: VecDeque<Box<dyn Transaction<()>>>,
    }
    impl TransactionSource<()> for TextParserTransactionSource {
        fn get_transaction(&mut self) -> Option<Box<dyn Transaction<()>>> {
            self.txs.pop_front()
        }
    }
    impl TextParserTransactionSource {
        pub fn new(db: MockDb, input: String) -> Self {
            let txs = transactions()
                .parse(&input)
                .map(|(ts, _)| {
                    ts.into_iter()
                        .map(|t| to_tx(t, db.clone()))
                        .collect::<VecDeque<_>>()
                })
                .unwrap_or_default();

            Self { txs }
        }
    }

    fn to_tx(command: Command, db: MockDb) -> Box<dyn Transaction<()>> {
        match command {
            Command::AddSalaryEmp {
                emp_id,
                name,
                address,
                salary,
            } => Box::new(AddSalaryEmployeeTxImpl {
                db,
                emp_id,
                name,
                address,
                salary,
            }),
            Command::AddHourlyEmp {
                emp_id,
                name,
                address,
                hourly_rate,
            } => Box::new(AddHourlyEmployeeTxImpl {
                db,
                emp_id,
                name,
                address,
                hourly_rate,
            }),
            Command::AddCommissionedEmp {
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            } => Box::new(AddCommissionedEmployeeTxImpl {
                db,
                emp_id,
                name,
                address,
                salary,
                commission_rate,
            }),
            Command::DelEmp { emp_id } => Box::new(DeleteEmployeeTxImpl { db, emp_id }),
            Command::TimeCard {
                emp_id,
                date,
                hours,
            } => Box::new(TimeCardTxImpl {
                db,
                emp_id,
                date,
                hours,
            }),
            Command::SalesReceipt {
                emp_id,
                date,
                amount,
            } => Box::new(SalesReceiptTxImpl {
                db,
                emp_id,
                date,
                amount,
            }),
            Command::ServiceCharge {
                member_id,
                date,
                amount,
            } => Box::new(ServiceChargeTxImpl {
                db,
                member_id,
                date,
                amount,
            }),
            Command::ChgName { emp_id, name } => {
                Box::new(ChangeEmployeeNameTxImpl { db, emp_id, name })
            }
            Command::ChgAddress { emp_id, address } => Box::new(ChangeEmployeeAddressTxImpl {
                db,
                emp_id,
                address,
            }),
            Command::ChgSalaried { emp_id, salary } => {
                Box::new(ChangeEmployeeSalariedTxImpl { db, emp_id, salary })
            }
            Command::ChgHourly {
                emp_id,
                hourly_rate,
            } => Box::new(ChangeEmployeeHourlyTxImpl {
                db,
                emp_id,
                hourly_rate,
            }),
            Command::ChgCommissioned {
                emp_id,
                salary,
                commission_rate,
            } => Box::new(ChangeEmployeeCommissionedTxImpl {
                db,
                emp_id,
                salary,
                commission_rate,
            }),
            Command::ChgHold { emp_id } => Box::new(ChangeEmployeeHoldTxImpl { db, emp_id }),
            Command::ChgDirect {
                emp_id,
                bank,
                account,
            } => Box::new(ChangeEmployeeDirectTxImpl {
                db,
                emp_id,
                bank,
                account,
            }),
            Command::ChgMail { emp_id, address } => Box::new(ChangeEmployeeMailTxImpl {
                db,
                emp_id,
                address,
            }),
            Command::ChgMember {
                emp_id,
                member_id,
                dues,
            } => Box::new(ChangeUnionMemberTxImpl {
                db,
                emp_id,
                member_id,
                dues,
            }),
            Command::ChgNoMember { emp_id } => Box::new(ChangeUnaffiliatedTxImpl { db, emp_id }),
            Command::Payday { pay_date } => Box::new(PaydayTxImpl { db, pay_date }),
        }
    }
}
use text_parser_tx_source::*;

mod mock_app {
    use std::path::PathBuf;

    use crate::mock_db::MockDb;
    use crate::text_parser_tx_source::TextParserTransactionSource;
    use crate::tx_app::{TransactionApplication, TransactionSource};

    #[derive(Debug, Clone)]
    pub struct TestPayrollApp {
        db: MockDb,
        file_path: PathBuf,
    }
    impl TestPayrollApp {
        pub fn new(file_name: &str) -> Self {
            Self {
                db: MockDb::new(),
                file_path: file_name.into(),
            }
        }
    }

    impl TransactionApplication<()> for TestPayrollApp {
        fn tx_source(&self) -> impl TransactionSource<()> {
            let input = std::fs::read_to_string(&self.file_path).expect("read script file");

            TextParserTransactionSource::new(self.db.clone(), input)
        }
    }
}
use mock_app::*;

mod parser {
    use chrono::NaiveDate;
    use parsec_rs::{char, float32, int32, keyword, pred, spaces, string, uint32, Parser};

    use crate::payroll_domain::EmployeeId;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Command {
        AddSalaryEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
        },
        AddHourlyEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            hourly_rate: f32,
        },
        AddCommissionedEmp {
            emp_id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,
        },
        DelEmp {
            emp_id: EmployeeId,
        },
        TimeCard {
            emp_id: EmployeeId,
            date: NaiveDate,
            hours: f32,
        },
        SalesReceipt {
            emp_id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        },
        ServiceCharge {
            member_id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        },
        ChgName {
            emp_id: EmployeeId,
            name: String,
        },
        ChgAddress {
            emp_id: EmployeeId,
            address: String,
        },
        ChgHourly {
            emp_id: EmployeeId,
            hourly_rate: f32,
        },
        ChgSalaried {
            emp_id: EmployeeId,
            salary: f32,
        },
        ChgCommissioned {
            emp_id: EmployeeId,
            salary: f32,
            commission_rate: f32,
        },
        ChgHold {
            emp_id: EmployeeId,
        },
        ChgDirect {
            emp_id: EmployeeId,
            bank: String,
            account: String,
        },
        ChgMail {
            emp_id: EmployeeId,
            address: String,
        },
        ChgMember {
            emp_id: EmployeeId,
            member_id: EmployeeId,
            dues: f32,
        },
        ChgNoMember {
            emp_id: EmployeeId,
        },
        Payday {
            pay_date: NaiveDate,
        },
    }
    pub fn transactions() -> impl Parser<Item = Vec<Command>> {
        transaction().many0()
    }
    pub fn transaction() -> impl Parser<Item = Command> {
        go_through().skip(
            add_salary_emp()
                .or(add_hourly_emp())
                .or(add_commissioned_emp())
                .or(del_emp())
                .or(time_card())
                .or(sales_receipt())
                .or(service_charge())
                .or(chg_name())
                .or(chg_address())
                .or(chg_hourly())
                .or(chg_salaried())
                .or(chg_commissioned())
                .or(chg_hold())
                .or(chg_direct())
                .or(chg_mail())
                .or(chg_member())
                .or(chg_no_member())
                .or(payday()),
        )
    }
    #[cfg(test)]
    mod test_transaction {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test_go_through() {
            let input = "";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "")));

            let input = "Code";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "Code")));

            let input = "# comment\nCode";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "Code")));

            let input = "# comment\n#\n# comment\nCode";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "Code")));

            let input = " \t\n# comment\n#\nCode";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "Code")));

            let input = " \t\n# comment\n#\n \tCode";
            let result = go_through().parse(input);
            assert_eq!(result, Ok(((), "Code")));
        }

        #[test]
        fn test_add_salary_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" S 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddSalaryEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_add_hourly_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" H 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddHourlyEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_add_commissioned_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" C 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddCommissionedEmp {
                        emp_id: 42,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_del_emp() {
            let input = r#"DelEmp 42"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Command::DelEmp { emp_id: 42 }, "")));
        }
        #[test]
        fn test_time_card() {
            let input = r#"TimeCard 42 2021-01-01 8.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::TimeCard {
                        emp_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        hours: 8.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_sales_receipt() {
            let input = r#"SalesReceipt 42 2021-01-01 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::SalesReceipt {
                        emp_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_service_charge() {
            let input = r#"ServiceCharge 42 2021-01-01 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ServiceCharge {
                        member_id: 42,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_name() {
            let input = r#"ChgEmp 42 Name "Bob""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgName {
                        emp_id: 42,
                        name: "Bob".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_address() {
            let input = r#"ChgEmp 42 Address "123 Wall St.""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgAddress {
                        emp_id: 42,
                        address: "123 Wall St.".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_hourly() {
            let input = r#"ChgEmp 42 Hourly 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgHourly {
                        emp_id: 42,
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_salaried() {
            let input = r#"ChgEmp 42 Salaried 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgSalaried {
                        emp_id: 42,
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_commissioned() {
            let input = r#"ChgEmp 42 Commissioned 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgCommissioned {
                        emp_id: 42,
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_hold() {
            let input = r#"ChgEmp 42 Hold"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Command::ChgHold { emp_id: 42 }, "")));
        }
        #[test]
        fn test_chg_direct() {
            let input = r#"ChgEmp 42 Direct "mufg" "1234567""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgDirect {
                        emp_id: 42,
                        bank: "mufg".to_string(),
                        account: "1234567".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_mail() {
            let input = r#"ChgEmp 42 Mail "bob@gmail.com""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgMail {
                        emp_id: 42,
                        address: "bob@gmail.com".to_string()
                    },
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_member() {
            let input = r#"ChgEmp 42 Member 7234 Dues 9.45"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgMember {
                        emp_id: 42,
                        member_id: 7234,
                        dues: 9.45,
                    },
                    "",
                ))
            );
        }
        #[test]
        fn test_no_member() {
            let input = r#"ChgEmp 42 NoMember"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Command::ChgNoMember { emp_id: 42 }, "")));
        }
    }

    fn go_through() -> impl Parser<Item = ()> {
        let comment = char('#').skip(pred(|c| c != '\n').many0().with(char('\n')));
        let space_comment = spaces().skip(comment).map(|_| ());
        let ignore = space_comment.many1().map(|_| ()).or(spaces().map(|_| ()));

        spaces().skip(ignore).skip(spaces()).map(|_| ())
    }

    fn add_salary_emp() -> impl Parser<Item = Command> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let monthly_rate = char('S').skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(monthly_rate)
            .map(
                |(((emp_id, name), address), salary)| Command::AddSalaryEmp {
                    emp_id,
                    name,
                    address,
                    salary,
                },
            )
    }
    #[cfg(test)]
    mod test_add_salary_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" S 1000.0"#;
            let result = add_salary_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddSalaryEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn add_hourly_emp() -> impl Parser<Item = Command> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let hourly_rate = char('H').skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(hourly_rate)
            .map(
                |(((emp_id, name), address), hourly_rate)| Command::AddHourlyEmp {
                    emp_id,
                    name,
                    address,
                    hourly_rate,
                },
            )
    }
    #[cfg(test)]
    mod test_add_hourly_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" H 1000.0"#;
            let result = add_hourly_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddHourlyEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        hourly_rate: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn add_commissioned_emp() -> impl Parser<Item = Command> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let salary = char('C').skip(spaces()).skip(float32()).with(spaces());
        let commission_rate = float32();

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(salary)
            .join(commission_rate)
            .map(|((((emp_id, name), address), salary), commission_rate)| {
                Command::AddCommissionedEmp {
                    emp_id,
                    name,
                    address,
                    salary,
                    commission_rate,
                }
            })
    }
    #[cfg(test)]
    mod test_add_commissioned_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" C 1000.0 0.1"#;
            let result = add_commissioned_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::AddCommissionedEmp {
                        emp_id: 1,
                        name: "Bob".to_string(),
                        address: "Home".to_string(),
                        salary: 1000.0,
                        commission_rate: 0.1
                    },
                    ""
                ))
            );
        }
    }

    fn del_emp() -> impl Parser<Item = Command> {
        let prefix = keyword("DelEmp").skip(spaces());
        let emp_id = uint32();

        prefix.skip(emp_id).map(|emp_id| Command::DelEmp { emp_id })
    }
    #[cfg(test)]
    mod test_del_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"DelEmp 1"#;
            let result = del_emp().parse(input);
            assert_eq!(result, Ok((Command::DelEmp { emp_id: 1 }, "")));
        }
    }

    fn date() -> impl Parser<Item = NaiveDate> {
        let year = int32().with(char('-'));
        let month = uint32().with(char('-'));
        let day = uint32();

        year.join(month)
            .join(day)
            .map(|((y, m), d)| NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32).expect("date"))
    }
    #[cfg(test)]
    mod test_date {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = "2021-01-01";
            let result = date().parse(input);
            assert_eq!(
                result,
                Ok((NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), ""))
            );
        }
    }

    fn time_card() -> impl Parser<Item = Command> {
        let prefix = keyword("TimeCard").skip(spaces());
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let hours = float32();

        prefix
            .skip(emp_id)
            .join(date)
            .join(hours)
            .map(|((emp_id, date), hours)| Command::TimeCard {
                emp_id,
                date,
                hours,
            })
    }
    #[cfg(test)]
    mod test_time_card {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"TimeCard 1 2021-01-01 8.0"#;
            let result = time_card().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::TimeCard {
                        emp_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        hours: 8.0
                    },
                    ""
                ))
            );
        }
    }

    fn sales_receipt() -> impl Parser<Item = Command> {
        let prefix = keyword("SalesReceipt").skip(spaces());
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32();

        prefix
            .skip(emp_id)
            .join(date)
            .join(amount)
            .map(|((emp_id, date), amount)| Command::SalesReceipt {
                emp_id,
                date,
                amount,
            })
    }
    #[cfg(test)]
    mod test_sales_receipt {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"SalesReceipt 1 2021-01-01 1000.0"#;
            let result = sales_receipt().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::SalesReceipt {
                        emp_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn service_charge() -> impl Parser<Item = Command> {
        let prefix = keyword("ServiceCharge").skip(spaces());
        let member_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32();

        prefix
            .skip(member_id)
            .join(date)
            .join(amount)
            .map(|((member_id, date), amount)| Command::ServiceCharge {
                member_id,
                date,
                amount,
            })
    }
    #[cfg(test)]
    mod test_service_charge {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ServiceCharge 1 2021-01-01 1000.0"#;
            let result = service_charge().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ServiceCharge {
                        member_id: 1,
                        date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        amount: 1000.0
                    },
                    ""
                ))
            );
        }
    }

    fn chg_name() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = keyword("Name").skip(spaces()).skip(string());

        prefix
            .skip(emp_id)
            .join(name)
            .map(|(emp_id, name)| Command::ChgName { emp_id, name })
    }
    #[cfg(test)]
    mod test_chg_name {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Name "Bob""#;
            let result = chg_name().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgName {
                        emp_id: 1,
                        name: "Bob".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_address() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Address").skip(spaces()).skip(string());

        prefix
            .skip(emp_id)
            .join(address)
            .map(|(emp_id, address)| Command::ChgAddress { emp_id, address })
    }
    #[cfg(test)]
    mod test_chg_address {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Address "123 Main St""#;
            let result = chg_address().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgAddress {
                        emp_id: 1,
                        address: "123 Main St".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_hourly() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hourly_rate = keyword("Hourly").skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(hourly_rate)
            .map(|(emp_id, hourly_rate)| Command::ChgHourly {
                emp_id,
                hourly_rate,
            })
    }
    #[cfg(test)]
    mod test_chg_hourly {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Hourly 13.78"#;
            let result = chg_hourly().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgHourly {
                        emp_id: 1,
                        hourly_rate: 13.78
                    },
                    ""
                ))
            );
        }
    }

    fn chg_salaried() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salaried = keyword("Salaried").skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(salaried)
            .map(|(emp_id, salary)| Command::ChgSalaried { emp_id, salary })
    }
    #[cfg(test)]
    mod test_chg_salaried {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Salaried 1023.456"#;
            let result = chg_salaried().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgSalaried {
                        emp_id: 1,
                        salary: 1023.456
                    },
                    ""
                ))
            );
        }
    }

    fn chg_commissioned() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salary = keyword("Commissioned")
            .skip(spaces())
            .skip(float32())
            .with(spaces());
        let commission_rate = float32();

        prefix.skip(emp_id).join(salary).join(commission_rate).map(
            |((emp_id, salary), commission_rate)| Command::ChgCommissioned {
                emp_id,
                salary,
                commission_rate,
            },
        )
    }
    #[cfg(test)]
    mod test_chg_commissioned {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Commissioned 1018.91 0.19"#;
            let result = chg_commissioned().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgCommissioned {
                        emp_id: 1,
                        salary: 1018.91,
                        commission_rate: 0.19
                    },
                    ""
                ))
            );
        }
    }

    fn chg_hold() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hold = keyword("Hold");

        prefix
            .skip(emp_id)
            .with(hold)
            .map(|emp_id| Command::ChgHold { emp_id })
    }
    #[cfg(test)]
    mod test_chg_hold {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Hold"#;
            let result = chg_hold().parse(input);
            assert_eq!(result, Ok((Command::ChgHold { emp_id: 1 }, "")));
        }
    }

    fn chg_direct() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let bank = keyword("Direct")
            .skip(spaces())
            .skip(string())
            .with(spaces());
        let account = string();

        prefix
            .skip(emp_id)
            .join(bank)
            .join(account)
            .map(|((emp_id, bank), account)| Command::ChgDirect {
                emp_id,
                bank,
                account,
            })
    }
    #[cfg(test)]
    mod test_chg_direct {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Direct "Bank" "Account""#;
            let result = chg_direct().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgDirect {
                        emp_id: 1,
                        bank: "Bank".to_string(),
                        account: "Account".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_mail() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Mail").skip(spaces()).skip(string());

        prefix
            .skip(emp_id)
            .join(address)
            .map(|(emp_id, address)| Command::ChgMail { emp_id, address })
    }
    #[cfg(test)]
    mod test_chg_mail {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Mail "bob@gmail.com""#;
            let result = chg_mail().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgMail {
                        emp_id: 1,
                        address: "bob@gmail.com".to_string()
                    },
                    ""
                ))
            );
        }
    }

    fn chg_member() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let member_id = keyword("Member")
            .skip(spaces())
            .skip(uint32())
            .with(spaces());
        let dues = keyword("Dues").skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(member_id)
            .join(dues)
            .map(|((emp_id, member_id), dues)| Command::ChgMember {
                emp_id,
                member_id,
                dues,
            })
    }
    #[cfg(test)]
    mod test_chg_member {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Member 2 Dues 100.0"#;
            let result = chg_member().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::ChgMember {
                        emp_id: 1,
                        member_id: 2,
                        dues: 100.0
                    },
                    ""
                ))
            );
        }
    }

    fn chg_no_member() -> impl Parser<Item = Command> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let no_member = keyword("NoMember");

        prefix
            .skip(emp_id)
            .with(no_member)
            .map(|emp_id| Command::ChgNoMember { emp_id })
    }
    #[cfg(test)]
    mod test_chg_no_member {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 NoMember"#;
            let result = chg_no_member().parse(input);
            assert_eq!(result, Ok((Command::ChgNoMember { emp_id: 1 }, "")));
        }
    }

    fn payday() -> impl Parser<Item = Command> {
        let prefix = keyword("Payday").skip(spaces());
        let date = date();

        prefix
            .skip(date)
            .map(|pay_date| Command::Payday { pay_date })
    }
    #[cfg(test)]
    mod test_payday {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"Payday 2021-01-01"#;
            let result = payday().parse(input);
            assert_eq!(
                result,
                Ok((
                    Command::Payday {
                        pay_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
                    },
                    ""
                ))
            );
        }
    }
}

fn main() -> Result<(), UsecaseError> {
    let mut app = TestPayrollApp::new("script/test.scr");
    app.run(&mut ())?;
    println!("{:#?}", app);

    Ok(())
}
