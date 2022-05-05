use anyhow::Context;
use clap::Parser;
use erldash::{metrics, ui};

/// Erlang Dashboard.
#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    #[clap(flatten)]
    options: erldash::Options,

    #[clap(hide = true, long)]
    logfile: Option<std::path::PathBuf>,

    #[clap(hide = true,long, default_value_t = simplelog::LevelFilter::Info)]
    loglevel: simplelog::LevelFilter,

    #[clap(hide = true, long)]
    truncate_log: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    setup_logger(&args)?;

    let (system_version, poller) = metrics::MetricsPoller::start_thread(args.options)?;
    let app = ui::App::new(system_version, poller)?;
    app.run()?;
    Ok(())
}

fn setup_logger(args: &Args) -> anyhow::Result<()> {
    if let Some(logfile) = &args.logfile {
        let file = std::fs::OpenOptions::new()
            .append(!args.truncate_log)
            .truncate(args.truncate_log)
            .create(true)
            .write(true)
            .open(logfile)
            .with_context(|| format!("failed to open log file {:?}", logfile))?;
        simplelog::WriteLogger::init(args.loglevel, Default::default(), file)?;
    }
    Ok(())
}
