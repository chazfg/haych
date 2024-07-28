use std::{fs, thread, time::Duration};
use std::sync::{Mutex, Arc, MutexGuard};
use std::{process, path::Path};
use crate::config::ProjectConfig;
use crate::build_cmd;
use crate::handle_tree;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};
use notify::{RecursiveMode, Watcher, Event, event::EventKind};
use notify_debouncer_full::new_debouncer;


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


/// Advanced example of the notify-debouncer-full, accessing the internal file ID cache
pub fn watch_files(args: WatchArgs) {
    // setup debouncer
    std::fs::create_dir("debug");
    let file_tree = handle_tree::get_tree();
    let file_tree_lock = Arc::new(Mutex::new(file_tree));


    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx).unwrap();
    debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| handle_event(&event.event, Arc::clone(&file_tree_lock))),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }

}

fn handle_event(event: &Event, ft_lock: Arc<Mutex<tera::Tera>>) {
    match event {
        Event {kind: EventKind::Modify(_), ..} => {
            let ft_lock_clone = Arc::clone(&ft_lock);
            thread::spawn(move|| {
                match ft_lock_clone.try_lock() {
                    Ok(mut lock) => {
                        lock.full_reload();
                        let bh = lock.render("layout.html", &tera::Context::new()).unwrap();
                        std::fs::write("debug/index.html", bh);                        
                },
                Err(_) => (println!("no lock"))
                }
            });
        },
        _ => ()
    }
}

#[derive(Debug)]
enum ChangeBuffer {
    Null,
    Rebuild
}

