use tx_rs::Tx;

use crate::error::UsecaseError;
use dao::{HavePayrollDao, PayrollDao};
use payroll_domain::{Employee, EmployeeId};

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
