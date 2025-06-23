use roboplc::controller::prelude::*;
use roboplc::prelude::*;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

type Message = ();
type Variables = ();

#[derive(WorkerOpts)]
#[worker_opts(cpu = 0, priority = 50, scheduling = "fifo", blocking = true)]
struct Worker1 {}

impl Worker<Message, Variables> for Worker1 {
    fn run(&mut self, _context: &Context<(), ()>) -> WResult {
        for (cycles, _) in roboplc::time::interval(Duration::from_millis(1)).enumerate() {
            if cycles % 10_000 == 0 {
                tracing::info!(cycles, worker = self.worker_name(), "stats");
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    roboplc::setup_panic();
    roboplc::configure_logger(roboplc::LevelFilter::Info);
    if !roboplc::is_production() {
        roboplc::set_simulated();
    }
    roboplc::thread_rt::prealloc_heap(10_000_000)?;
    let mut controller = Controller::<Message, Variables>::new();
    controller.spawn_worker(Worker1 {})?;
    controller.register_signals(SHUTDOWN_TIMEOUT)?;
    controller.block();
    Ok(())
}
