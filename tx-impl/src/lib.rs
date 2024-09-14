use chrono::NaiveDate;
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use abstract_tx::{
    AddEmployeeTx, ChangeAffiliationTx, ChangeEmployeePaymentClassificationTx,
    ChangeEmployeePaymentMethodTx, ChangeEmployeeTx, UsecaseError,
};
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::{EmployeeId, MemberId, Paycheck};
use payroll_impl::{
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

pub trait ChangeEmployeeCommissionedTx<Ctx>: ChangeEmployeePaymentClassificationTx<Ctx> {
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
