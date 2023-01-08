use tide::Request;
use clap::Parser;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub id: u64,
    #[arg(long)]
    pub rb: u64,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let state = Arc::new(Mutex::new(args.id));
    let mut app = tide::with_state(state);

    app.at("/get-id")
        .get(move |req: Request<Arc<Mutex<u64>>>| async move {
            let state = req.state();
            let mut guard = state.lock().unwrap();

            if *guard < args.rb {
                *guard += 1;
                Ok(format!("{guard}"))
            } else {
                Ok(format!("too many ids were generated"))
            }
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}