use std::collections::VecDeque;

use crate::command::Command;
use crate::parser::read_commands;
use tx_app::{Transaction, TransactionSource};
use tx_factory::TransactionFactory;

pub struct TextParserTransactionSource<Ctx, F>
where
    F: TransactionFactory<Ctx>,
{
    tx_factory: F,
    commands: VecDeque<Command>,
    phantom: std::marker::PhantomData<Ctx>,
}
impl<Ctx, F> TransactionSource<Ctx> for TextParserTransactionSource<Ctx, F>
where
    F: TransactionFactory<Ctx>,
{
    fn get_transaction(&mut self) -> Option<Box<dyn Transaction<Ctx> + '_>> {
        self.commands
            .pop_front()
            .map(|c| c.convert(&self.tx_factory))
    }
}
impl<Ctx, F> TextParserTransactionSource<Ctx, F>
where
    F: TransactionFactory<Ctx>,
{
    pub fn new(tx_factory: F, input: String) -> Self {
        let commands = read_commands(&input);

        Self {
            tx_factory,
            commands,
            phantom: std::marker::PhantomData,
        }
    }
}
