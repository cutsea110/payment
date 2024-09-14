use abstract_tx::UsecaseError;

pub trait Transaction<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<(), UsecaseError>;
}
