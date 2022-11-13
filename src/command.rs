use log::{error, warn};

pub enum Command {
    Workspace(usize),
    ActiveWindow {
        title: String,
        class: String,
    },
    CreateWorkspace(usize),
    DestroyWorkspace(usize),
    FocusedMon {
        monitor: String,
        workspace: usize,
    },
    FullScreen(bool),
    MonitorAdded(String),
    MonitorRemoved(String),
    MoveWorkspace {
        workspace: usize,
        monitor: String,
    },
    OpenWindow {
        address: String,
        class: String,
        title: String,
        workspace: usize,
    },
    CloseWindow(String),
    MoveWindow {
        address: String,
        workspace: usize,
    },
    OpenLayer(String),
    CloseLayer(String),
    SubMap(String),
}

pub enum Error {
    ParseFailed,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::ParseFailed
    }
}

impl std::str::FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd, param)) = s.trim().split_once(">>") {
            match cmd {
                "workspace" => {
                    if let Ok(id) = param.parse::<usize>() {
                        Ok(Self::Workspace(id))
                    } else {
                        error!("Invalid workspace ({param})");
                        Err(Error::ParseFailed)
                    }
                }
                "createworkspace" => {
                    if let Ok(id) = param.parse::<usize>() {
                        Ok(Self::CreateWorkspace(id))
                    } else {
                        error!("Invalid workspace ({param})");
                        Err(Error::ParseFailed)
                    }
                }
                "destroyworkspace" => {
                    if let Ok(id) = param.parse::<usize>() {
                        Ok(Self::DestroyWorkspace(id))
                    } else {
                        error!("Invalid workspace ({param})");
                        Err(Error::ParseFailed)
                    }
                }
                "activewindow" => {
                    if let Some((class, title)) = param.split_once(',') {
                        Ok(Self::ActiveWindow {
                            class: class.to_string(),
                            title: title.to_string(),
                        })
                    } else {
                        error!("Invalid window ({param})");
                        Err(Error::ParseFailed)
                    }
                }
                "focusedmon" => {
                    if let Some((class, workspace)) = param.split_once(',') {
                        Ok(Self::FocusedMon {
                            monitor: class.to_string(),
                            workspace: if let Ok(w) = workspace.parse() {
                                w
                            } else {
                                error!("Couldn't parse workspace {}", workspace);
                                return Err(Error::ParseFailed);
                            },
                        })
                    } else {
                        error!("Invalid window ({param})");
                        Err(Error::ParseFailed)
                    }
                }
                "fullscreen" => match param.parse() {
                    Ok(0) => Ok(Self::FullScreen(false)),
                    Ok(1) => Ok(Self::FullScreen(true)),
                    _ => {
                        error!("Couldn't parse full screen info: {}", param);
                        Err(Error::ParseFailed)
                    }
                },
                "monitoradded" => Ok(Self::MonitorAdded(param.to_string())),
                "monitorremoved" => Ok(Self::MonitorRemoved(param.to_string())),
                "moveworkspace" => {
                    if let Some((workspace, monitor)) = param.split_once(',') {
                        if let Ok(workspace) = workspace.parse() {
                            return Ok(Self::MoveWorkspace {
                                workspace,
                                monitor: monitor.to_string(),
                            });
                        }
                    }
                    Err(Error::ParseFailed)
                }
                "openwindow" => {
                    let (address, remain) = param.split_once(',').ok_or(Error::ParseFailed)?;
                    let (workspace, remain) = remain.split_once(',').ok_or(Error::ParseFailed)?;
                    let (class, title) = remain.split_once(',').ok_or(Error::ParseFailed)?;
                    Ok(Self::OpenWindow {
                        address: address.to_string(),
                        class: class.to_string(),
                        title: title.to_string(),
                        workspace: workspace.parse()?,
                    })
                }
                "closewindow" => Ok(Self::CloseWindow(param.to_string())),
                "movewindow" => {
                    if let Some((address, workspace_name)) = param.split_once(',') {
                        if let Ok(workspace) = workspace_name.parse() {
                            return Ok(Self::MoveWindow {
                                address: address.to_string(),
                                workspace,
                            });
                        }
                    }
                    Err(Error::ParseFailed)
                }
                "openlayer" => Ok(Self::OpenLayer(param.to_string())),
                "closelayer" => Ok(Self::CloseLayer(param.to_string())),
                "submap" => Ok(Self::SubMap(param.to_string())),
                _ => {
                    warn!("invalid Command ({})", cmd);
                    Err(Error::ParseFailed)
                }
            }
        } else {
            warn!("Couldn't parse ({})", s);
            Err(Error::ParseFailed)
        }
    }
}
