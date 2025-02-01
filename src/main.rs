use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use rgb_daemon::{
    Command, RgbCommand, Profile, ColorSetting,
    RgbController, MoteController, MoteState
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the RGB daemon
    Daemon {
        #[arg(short, long, default_value = "/tmp/rgb-daemon.sock")]
        socket: PathBuf,
    },
    /// Send a command to the daemon
    Set {
        #[arg(short, long)]
        red: u8,
        #[arg(short, long)]
        green: u8,
        #[arg(short, long)]
        blue: u8,
        #[arg(short, long, default_value = "/tmp/rgb-daemon.sock")]
        socket: PathBuf,
    },
    /// Activate a preset profile
    Profile {
        #[arg(value_enum)]
        profile: Profile,
        #[arg(short, long, default_value = "/tmp/rgb-daemon.sock")]
        socket: PathBuf,
    },
}

struct DaemonState {
    controller: Box<dyn RgbController>,
    state: MoteState,
}

impl DaemonState {
    fn new() -> Result<Self> {
        Ok(Self {
            controller: Box::new(MoteController::new("Pimoroni Mote".to_string())?),
            state: MoteState::default(),
        })
    }

    async fn restore_state(&mut self) -> Result<()> {
        if let Some(color) = &self.state.last_color {
            self.controller.set_color(color.red, color.green, color.blue)?;
        } else if let Some(profile) = &self.state.current_profile {
            if let Some(mote) = self.controller.as_any().downcast_mut::<MoteController>() {
                mote.transition_to(profile.clone()).await?;
            }
        }
        Ok(())
    }
}

async fn run_daemon(socket_path: PathBuf) -> Result<()> {
    // Remove existing socket file if it exists
    _ = std::fs::remove_file(&socket_path);
    
    // Create Unix domain socket
    let listener = UnixListener::bind(&socket_path)?;
    println!("Daemon listening on {:?}", socket_path);

    let state = Arc::new(Mutex::new(DaemonState::new()?));
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = state.clone();
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    if let Ok(command) = serde_json::from_slice::<Command>(&buffer[..n]) {
                        let mut state = state.lock().await;
                        match command {
                            Command::SetColor(rgb) => {
                                println!("Daemon received SetColor command: RGB({}, {}, {})", 
                                    rgb.red, rgb.green, rgb.blue);
                                if let Err(e) = state.controller.set_color(
                                    rgb.red,
                                    rgb.green,
                                    rgb.blue,
                                ) {
                                    eprintln!("Error setting color: {}", e);
                                } else {
                                    state.state.current_profile = None;
                                    state.state.last_color = Some(rgb);
                                }
                            }
                            Command::SetProfile(profile) => {
                                println!("Daemon received SetProfile command: {:?}", profile);
                                if let Some(mote) = state.controller.as_any().downcast_mut::<MoteController>() {
                                    println!("Starting transition to profile...");
                                    if let Err(e) = mote.transition_to(profile.clone()).await {
                                        eprintln!("Error transitioning to profile: {}", e);
                                    } else {
                                        state.state.current_profile = Some(profile);
                                        state.state.last_color = None;
                                        println!("Transition complete");
                                    }
                                } else {
                                    eprintln!("Controller doesn't support profiles");
                                }
                            }
                            Command::Reconnect => {
                                println!("Daemon received Reconnect command");
                                // Create new controller instance
                                match MoteController::new("Pimoroni Mote".to_string()) {
                                    Ok(new_controller) => {
                                        state.controller = Box::new(new_controller);
                                        println!("Successfully reconnected to device");
                                        
                                        // Restore previous state
                                        if let Err(e) = state.restore_state().await {
                                            eprintln!("Failed to restore previous state: {}", e);
                                        } else {
                                            println!("Successfully restored previous state");
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to reconnect to device: {}", e);
                                    }
                                }
                            }
                        }
                    } else {
                        eprintln!("Failed to parse command from JSON");
                        if let Ok(str_data) = std::str::from_utf8(&buffer[..n]) {
                            eprintln!("Received data: {}", str_data);
                        }
                    }
                }
                Err(e) => eprintln!("Error reading from socket: {}", e),
            }
        });
    }
}

async fn send_command(socket_path: PathBuf, command: Command) -> Result<()> {
    let mut stream = tokio::net::UnixStream::connect(socket_path).await?;
    let command_json = serde_json::to_vec(&command)?;
    stream.write_all(&command_json).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Daemon { socket } => {
            println!("Starting daemon...");
            run_daemon(socket).await?;
        }
        Commands::Set { red, green, blue, socket } => {
            println!("Setting color to RGB({}, {}, {})", red, green, blue);
            send_command(socket, Command::SetColor(RgbCommand { red, green, blue })).await?;
        }
        Commands::Profile { profile, socket } => {
            println!("Activating profile: {:?}", profile);
            send_command(socket, Command::SetProfile(ColorSetting::from(profile))).await?;
        }
    }

    Ok(())
}
