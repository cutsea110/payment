use abstract_tx::UsecaseError;

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
