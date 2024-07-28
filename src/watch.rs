use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use std::path::Path;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Event, Config};

pub fn watch_files(args: WatchArgs) {

    futures::executor::block_on(async {
        if let Err(e) = async_watch(".").await {
            println!("error: {:?}", e)
        }
    });

}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WatchArgs {
    #[arg(long, default_value="test")]
    test: String,

}


pub fn watch_command() -> Command {
    Command::new("watch")
        .long_about("watches for changes")

}


fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}