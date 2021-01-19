use log::{info, warn, error, debug};


fn main() {
    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Information message");
    debug!("Debugging message");
    info!(target: "yak_events", "Commencing yak shaving for");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {}", razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {}, retrying", err);
                break;
            }
        }
    }
}

fn find_a_razor() -> Result<i32, i32> {
    Err(32)
}
