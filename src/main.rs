use clap::{Parser, Subcommand, ValueEnum};
use hyprland::{
    data::{blocking::*, Monitor},
    dispatch::{
        dispatch_blocking, DispatchType, MonitorIdentifier, WindowMove, WorkspaceIdentifier,
        WorkspaceIdentifierWithSpecial,
    },
    event_listener::EventListener,
    shared::WorkspaceType,
};

// All the clap code

#[derive(Subcommand, Debug, Clone)]
enum WorkspaceCommand {
    /// Focus the appropriate workspace
    Focus { workspace: u8 },
    /// Move the focused window to the appropriate workspace
    Move { workspace: u8 },
}

#[derive(Subcommand, Debug, Clone)]
enum DisplayCommand {
    /// Focus the next monitor in provided direction, will loop
    Focus { direction: Direction },
    /// Move the focused window to the monitor in the provided direction, will loop
    Move { direction: Direction },
    /// Listen for new monitors and set up workspaces for it
    Listener,
}

#[derive(ValueEnum, Debug, Clone)]
enum Direction {
    L,
    R,
}

#[derive(Subcommand, Debug, Clone)]
enum BarCommand {
    /// Subscribe to a workspace status, will return waybar compatible JSON
    Workspace { workspace: u8, display: u8 },
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Commands for interacting with workspaces
    #[command(subcommand)]
    Workspace(WorkspaceCommand),
    /// Commands for interacting with displays
    #[command(subcommand)]
    Display(DisplayCommand),
    /// Commands for bar stuff
    #[command(subcommand)]
    Bar(BarCommand),
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> std::io::Result<()> {
    // Get the args via clap
    let cli = Cli::parse();

    match cli.command {
        Commands::Workspace(workspace) => {
            let monitor = get_active_monitor()?;
            match workspace {
                WorkspaceCommand::Focus { workspace } => {
                    if monitor.id == 0 {
                        dispatch_blocking(DispatchType::Workspace(
                            WorkspaceIdentifierWithSpecial::Id(workspace),
                        ))?;
                    } else {
                        dispatch_blocking(DispatchType::Workspace(
                            WorkspaceIdentifierWithSpecial::Id(
                                format!("{}{}", monitor.id, workspace).parse().unwrap(),
                            ),
                        ))?;
                    }
                }
                WorkspaceCommand::Move { workspace } => {
                    if monitor.id == 0 {
                        dispatch_blocking(DispatchType::MoveFocusedWindowToWorkspace(
                            WorkspaceIdentifier::Id(workspace),
                        ))?;
                    } else {
                        dispatch_blocking(DispatchType::MoveFocusedWindowToWorkspace(
                            WorkspaceIdentifier::Id(
                                format!("{}{}", monitor.id, workspace).parse().unwrap(),
                            ),
                        ))?;
                    }
                }
            }
        }
        Commands::Display(display) => match display {
            DisplayCommand::Focus { direction } => dispatch_blocking(DispatchType::FocusMonitor(
                MonitorIdentifier::Id(get_monitor(direction)?.id),
            ))?,
            DisplayCommand::Move { direction } => dispatch_blocking(DispatchType::MoveWindow(
                WindowMove::Monitor(MonitorIdentifier::Id(get_monitor(direction)?.id)),
            ))?,
            DisplayCommand::Listener => {
                let mut listener = EventListener::new();

                listener.add_monitor_added_handler(|name| {
                    let monitors = match get_monitors() {
                        Ok(mons) => mons,
                        Err(why) => {
                            println!("Error getting monitors: {}", why);
                            return;
                        }
                    };
                    let monitor = match monitors.iter().find(|monitor| monitor.name == name) {
                        Some(monitor) => monitor,
                        None => {
                            println!("Monitor not present!");
                            return;
                        }
                    };

                    for i in 1..=9 {
                        match if monitor.id != 0 {
                            dispatch_blocking(DispatchType::Keyword(
                                "wsbind".to_string(),
                                format!("{}{},{}", monitor.id, i, name),
                            ))
                        } else {
                            dispatch_blocking(DispatchType::Keyword(
                                "wsbind".to_string(),
                                format!("{},{}", i, name),
                            ))
                        } {
                            Ok(_) => (),
                            Err(why) => {
                                println!("Error binding workspace: {}", why);
                            }
                        }
                    }

                    match if monitor.id != 0 {
                        dispatch_blocking(DispatchType::Keyword(
                            "workspace".to_string(),
                            format!("{},{}1", name, monitor.id),
                        ))
                    } else {
                        dispatch_blocking(DispatchType::Keyword(
                            "workspace".to_string(),
                            format!("{},1", name),
                        ))
                    } {
                        Ok(_) => (),
                        Err(why) => {
                            println!("Error setting workspace: {}", why);
                        }
                    }
                });
            }
        },
        Commands::Bar(bar) => match bar {
            BarCommand::Workspace { workspace, display } => {
                let mut listener = EventListener::new();

                let ws_workspace = format!("{}{}", display, workspace).parse::<u8>().unwrap();

                let handler = move |ws: WorkspaceType| match ws {
                    WorkspaceType::Regular(id) => {
                        if id == ws_workspace {
                            println!(r#"{{ "text": " {} ", "class": ["selected"] }}"#, workspace);
                        } else {
                            println!(r#"{{ "text": " {} ", "class": [""] }}"#, workspace);
                        }
                    }
                    WorkspaceType::Special => (),
                };

                listener.add_workspace_change_handler(move |ws| handler(ws));
                listener.add_active_monitor_change_handler(move |data| handler(data.1));

                listener.start_listener_blocking()?;
            }
        },
    }

    Ok(())
}

/// Helper to get the next monitor using the provided direction, will loop
fn get_monitor(direction: Direction) -> std::io::Result<Monitor> {
    let mut monitors = get_monitors()?;
    let monitor = get_active_monitor()?;

    monitors.sort_by(|a, b| a.x.cmp(&b.x));
    let index = monitors
        .iter()
        .position(|mon| mon.id == monitor.id)
        .unwrap();

    Ok(match direction {
        Direction::R => {
            if index < monitors.len() - 1 {
                monitors[index + 1].clone()
            } else {
                monitors[0].clone()
            }
        }
        Direction::L => {
            if index > 0 {
                monitors[index - 1].clone()
            } else {
                monitors[monitors.len() - 1].clone()
            }
        }
    })
}
