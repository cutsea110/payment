use std::{cell::RefCell, rc::Rc};

use abstract_tx::{ChangeEmployeePaymentClassificationTx, UsecaseError};
use payroll_domain::EmployeeId;
use payroll_impl::{PaymentClassificationImpl, PaymentScheduleImpl};

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
