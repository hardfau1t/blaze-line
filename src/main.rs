#![allow(unreachable_code)]
use self::command::Command;
use clap::Parser;
use log::{debug, info};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, action=clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    quiet: bool,
}

mod command;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::builder()
        .filter_level({
            if args.quiet {
                log::LevelFilter::Off
            } else {
                match args.verbose {
                    0 => log::LevelFilter::Error,
                    1 => log::LevelFilter::Warn,
                    2 => log::LevelFilter::Info,
                    3 => log::LevelFilter::Debug,
                    _ => log::LevelFilter::Trace,
                }
            }
        })
        .init();
    let mut hypr_sock = std::path::PathBuf::from("/tmp/hypr");
    hypr_sock.push(std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap());
    hypr_sock.push(".socket2.sock");
    debug!("socket path: {:?}", hypr_sock);
    let mut stream = std::io::BufReader::new(UnixStream::connect(hypr_sock)?);
    let mut response = String::with_capacity(100);
    loop {
        response.clear();
        if let Ok(msg_len) = stream.read_line(&mut response) {
            debug!("read {} bytes, msg: '{}'", msg_len, &response);
            if let Ok(cmd) = response.trim().parse::<Command>() {
                handle(&cmd);
            }
        }
    }
}

fn handle(cmd: &Command) {
    match cmd {
        Command::Workspace(id) => info!("Workspace {id}"),
        Command::ActiveWindow { class, title } => {
            info!("Switched to window {class} : {title}")
        }
        Command::CreateWorkspace(id) => info!("WorkSpace Created: {id}"),
        Command::DestroyWorkspace(id) => info!("WorkSpace Destroyed: {id}"),
        Command::FocusedMon { monitor, workspace } => {
            info!("Changed Monitor to {monitor}: {workspace}")
        }
        Command::FullScreen(mode) => info!("Full screen mode : {}", mode),
        Command::MonitorAdded(_) => todo!(),
        Command::MonitorRemoved(_) => todo!(),
        Command::MoveWorkspace { workspace, monitor } => info!("Workspace {workspace} moved to monitor {monitor}"),
        Command::OpenWindow { address, class, title, workspace } => info!("Window {title}-{class} = {address} Opened in workspace {workspace}"),
        Command::CloseWindow(address) => info!("Closed Window {address}"),
        Command::MoveWindow { address, workspace } => info!("Window {address} moved to workspace {workspace}"),
        Command::OpenLayer(layer) => info!("Layer {layer} Opened"),
        Command::CloseLayer(layer) => info!("Layer {layer} closed"),
        Command::SubMap(smap) => info!("Submap event {smap}"),
    }
}
