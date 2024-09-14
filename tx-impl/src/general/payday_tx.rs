use chrono::NaiveDate;
use tx_rs::Tx;

use abstract_tx::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::Paycheck;

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
