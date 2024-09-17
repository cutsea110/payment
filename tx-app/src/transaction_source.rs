use crate::transaction::Transaction;

pub trait TransactionSource<Ctx> {
    fn get_transaction(&mut self) -> Option<Box<dyn Transaction<Ctx> + '_>>;
}
