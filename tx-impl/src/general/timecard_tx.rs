use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::EmployeeId;
use payroll_impl::{PaymentClassificationImpl, TimeCard};

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
