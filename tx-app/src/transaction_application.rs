use abstract_tx::UsecaseError;

use crate::transaction_source::TransactionSource;

pub trait TransactionApplication<Ctx> {
    fn tx_source(&self) -> impl TransactionSource<Ctx>;
    fn run(&mut self, ctx: &mut Ctx) -> Result<(), UsecaseError> {
        let mut tx_source = self.tx_source();
        while let Some(tx) = tx_source.get_transaction() {
            tx.execute(ctx)?;
        }
        Ok(())
    }
}
