use std::{fs, thread, time::Duration};
use std::sync::{Mutex, Arc, MutexGuard};
use std::{process, path::{Path, PathBuf}};
use crate::config::ProjectConfig;
use crate::build_cmd;
use crate::handle_tree;
use clap::error::{Error, ErrorKind};
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches, arg, value_parser};
use notify::{RecursiveMode, Watcher, Event, event::EventKind, event::EventAttributes};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, DebounceEventHandler, DebouncedEvent};
use std::sync::mpsc::Sender;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WatchArgs {
    #[arg(short, long, group="watch")]
    templates: Option<String>,
    #[arg(short, long, group="watch")]
    server: Option<String>,
}



pub fn watch_command() -> Command {
    Command::new("watch")
        .about("watch for changes")
        .args([
            arg!(-s --server <SERVER_PATH> "path to watch for server changes")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            arg!(-t --templates <TEMPLATE_PATH> "path to watch for template_changes")
                .required(false)
                .value_parser(value_parser!(PathBuf)),                
            ])
}



pub fn watch_files(args: &ArgMatches, _config: ProjectConfig) {
    
    let file_tree = handle_tree::get_tree();
    let bh = file_tree.render("layout.html", &tera::Context::new()).unwrap();
    std::fs::create_dir("debug");
    std::fs::write("debug/index.html", bh)
        .expect("should write");
    let file_tree_lock = Arc::new(Mutex::new(file_tree));

    
    let (tx, rx) = std::sync::mpsc::channel();
    
    let html_sitter = DirSitter(tx.clone(), String::from("html"));
    let cargo_sitter = DirSitter(tx.clone(), String::from("cargo"));
    
    let mut html_debouncer = new_debouncer(Duration::from_millis(200), None, html_sitter).unwrap();
    let mut cargo_debouncer = new_debouncer(Duration::from_millis(200), None, cargo_sitter).unwrap();
    
    let html_dir = Path::new("templates/");
    html_debouncer.watcher().watch(html_dir, RecursiveMode::Recursive).unwrap();

    let cargo_dir = Path::new("src/");
    cargo_debouncer.watcher().watch(cargo_dir, RecursiveMode::Recursive).unwrap();

    let mut child = process::Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("failed cargo build");

    let child_lock = Arc::new(Mutex::new(child));
    
    println!("Watching for front end changes in {}", html_dir.display());
    println!("Watching for back end changes in {}", cargo_dir.display());
    // // print all events and errors
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| {
                // println!("{:?}", event);
                match event.attrs.info() {
                    Some("html") => handle_event(&event.event, Arc::clone(&file_tree_lock)),
                    Some("cargo") => handle_cargo(&event.event, Arc::clone(&child_lock)),
                    _ => ()
                }
                
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }

}

fn handle_cargo(event: &Event, child_lock: Arc<Mutex<std::process::Child>>) {

    match event {
        Event {kind: EventKind::Modify(_), ..} => {
            println!("cargo event");
            let child_lock_clone = Arc::clone(&child_lock);
            thread::spawn(move|| {

                match child_lock_clone.try_lock() {
                    Ok(mut lock) => {
                        lock.kill().expect("could not kill child");

                        *lock = process::Command::new("cargo")
                            .arg("run")
                            .spawn()
                            .expect("failed cargo build");
                        },
                    Err(_) => println!("no lock cargo")
                    }
                });
                },
        _ => ()
    }
}

fn handle_event(event: &Event, ft_lock: Arc<Mutex<tera::Tera>>) {
    // println!("{:?}", event);
    match event {
        Event {kind: EventKind::Modify(_), ..} => {
            println!("html event");
            let ft_lock_clone = Arc::clone(&ft_lock);
            thread::spawn(move|| {

                match ft_lock_clone.try_lock() {
                    Ok(mut lock) => {
                        lock.full_reload()
                            .expect("should reload");
                        let bh = lock.render("layout.html", &tera::Context::new()).unwrap();
                        std::fs::write("debug/index.html", bh)
                            .expect("should write");
                        },
                    Err(_) => println!("no lock")
                    }
                });
                },
        _ => ()
    }
}


struct DirSitter(Sender<DebounceEventResult>, String);
impl DebounceEventHandler for DirSitter {
    fn handle_event(&mut self, result: DebounceEventResult) {
        self.0.send(
            result.map(|mut r| {
                r
                    .iter_mut()
                    .for_each(|der| der.event.attrs.set_info(&self.1));
                r
            })
            );
    }
}

