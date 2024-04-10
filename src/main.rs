use error_stack::{IntoReportCompat, ResultExt};

fn eyre_result() -> eyre::Result<()> {
    Err(eyre::eyre!("Something happened"))
}

#[derive(Debug, thiserror::Error)]
#[error("my error occurred")]
struct MyError;

fn main() -> error_stack::Result<(), MyError> {
    // doesn't compile
    eyre_result()
        .into_report()
        .change_context(MyError)?;

    // fine
    eyre_result()
        .into_report()
        .map_err(|report| report.change_context(MyError))?;


    Ok(())
}