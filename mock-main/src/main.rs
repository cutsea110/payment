use abstract_tx::UsecaseError;
use mock_app::*;
use tx_app::*;

fn main() -> Result<(), UsecaseError> {
    let mut app = TestPayrollApp::new("script/test.scr");
    app.run(&mut ())?;
    println!("{:#?}", app);

    Ok(())
}
