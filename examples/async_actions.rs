#[cfg(feature = "async")]
use async_trait::async_trait;
#[cfg(feature = "async")]
use clark::{App, AppIdentity, AppVersion, Arg, AsyncActionBuilder, AsyncActionHandler};
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::sync::Arc;
#[cfg(feature = "async")]
use std::task::{Context, Poll, Wake, Waker};

#[cfg(feature = "async")]
struct NoopWake;

#[cfg(feature = "async")]
impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

#[cfg(feature = "async")]
fn block_on<F: Future>(future: F) -> F::Output {
    // Minimal executor for running the example without a runtime dependency.
    let waker = Waker::from(Arc::new(NoopWake));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[cfg(feature = "async")]
struct GreetAction;

#[cfg(feature = "async")]
#[async_trait]
impl AsyncActionHandler for GreetAction {
    async fn run(&mut self, app: &mut App) {
        app.add_argument(
            "--name",
            Arg::new()
                .help("Who to greet")
                .with_default("world")
                .require_value()
                .optional(),
        );
        app.add_argument(
            "--loud",
            Arg::new()
                .help("Uppercase the greeting")
                .as_flag()
                .optional(),
        );
        app.parse_args(true);

        let name = app.args().first_of("--name").map(String::as_str).unwrap_or("world");
        let mut greeting = format!("Hello, {name}!");
        if app.args().contains("--loud") {
            greeting = greeting.to_uppercase();
        }

        println!("{greeting}");
    }
}

#[cfg(feature = "async")]
struct StatusAction;

#[cfg(feature = "async")]
#[async_trait]
impl AsyncActionHandler for StatusAction {
    async fn run(&mut self, app: &mut App) {
        app.add_argument(
            "--verbose",
            Arg::new()
                .help("Include extra details in the output")
                .as_flag()
                .optional(),
        );
        app.parse_args(true);

        let status = fetch_status().await;
        if app.args().contains("--verbose") {
            println!("Status: {status} (async handler)");
        } else {
            println!("{status}");
        }
    }
}

#[cfg(feature = "async")]
async fn fetch_status() -> &'static str {
    "All systems nominal"
}

#[cfg(feature = "async")]
async fn async_main() {
    let identity = AppIdentity::new(
        "Async Actions",
        "Demonstrates AsyncActionBuilder with async handlers.",
        AppVersion::new(0, 1, 0),
    );
    let mut app = App::new(identity);
    app.add_help_arguments();

    AsyncActionBuilder::new(&mut app, Some("Choose an action to run".into()))
        .add_action("greet", "Print a greeting", GreetAction)
        .add_action("status", "Show a status line", StatusAction)
        .run()
        .await;
}

#[cfg(feature = "async")]
fn main() {
    block_on(async_main());
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("Enable the async feature to run this example.");
    eprintln!("Example: cargo run --example async_actions --features async -- greet");
}
