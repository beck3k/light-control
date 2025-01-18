use anyhow::{Result, Context};
use rgb_daemon::{Command, Profile, ColorSetting};
use std::path::PathBuf;
use tray_icon::{Icon, TrayIconBuilder};
use tray_icon::menu::{Menu, MenuEvent, MenuItem, MenuId, accelerator::Accelerator};
use winit::event_loop::{ControlFlow, EventLoop};
use image;

const SOCKET_PATH: &str = "/tmp/rgb-daemon.sock";
const ICON_ON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/light_on.png");
const ICON_OFF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/light_off.png");

// Define our menu items
struct MenuItemDef {
    id: &'static str,
    label: &'static str,
    profile: Option<Profile>,
}

const MENU_ITEMS: &[MenuItemDef] = &[
    MenuItemDef { id: "profile_off", label: "Off", profile: Some(Profile::Off) },
    MenuItemDef { id: "profile_red", label: "Red", profile: Some(Profile::Red) },
    MenuItemDef { id: "profile_white", label: "White", profile: Some(Profile::White) },
    MenuItemDef { id: "quit", label: "Quit", profile: None },
];

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let icon = load_icon(ICON_OFF_PATH)?;

    // Create menu
    let menu = Menu::new();
    
    // Add all menu items
    for item_def in MENU_ITEMS {
        let menu_item = MenuItem::with_id(
            item_def.id,
            MenuId(item_def.label.to_string()),
            true,
            None::<Accelerator>
        );
        menu.append(&menu_item)?;
    }

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("RGB Controller")
        .with_icon(icon)
        .build()?;

    // Handle menu events
    let menu_channel = MenuEvent::receiver();
    
    // Create a channel for icon updates
    let (icon_tx, icon_rx) = std::sync::mpsc::channel();
    
    std::thread::spawn(move || {
        while let Ok(event) = menu_channel.recv() {            
            // Find the matching menu item definition
            if let Some(item_def) = MENU_ITEMS.iter().find(|item| item.id == event.id.0) {
                match item_def.profile {
                    Some(profile) => {
                        println!("Switching to {:?} profile", profile);
                        // Send icon path update through channel instead of directly updating
                        let icon_path = match profile {
                            Profile::Off => ICON_OFF_PATH,
                            _ => ICON_ON_PATH,
                        };
                        let _ = icon_tx.send(icon_path);
                        send_profile(profile);
                    }
                    None => {
                        println!("Exiting application");
                        std::process::exit(0);
                    }
                }
            } else {
                println!("Unknown menu item: {}", event.id.0);
            }
        }
    });

    event_loop.run(move |_event, elwt| {
        // Check for icon updates
        if let Ok(icon_path) = icon_rx.try_recv() {
            if let Ok(new_icon) = load_icon(icon_path) {
                let _ = tray_icon.set_icon(Some(new_icon));
            }
        }
        elwt.set_control_flow(ControlFlow::Wait);
    })?;

    Ok(())
}

fn load_icon(icon_path: &str) -> Result<Icon> {
    let image = image::open(icon_path)
        .context("Failed to open icon file")?
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    let icon = Icon::from_rgba(rgba, width, height)
        .context("Failed to create icon from RGBA data")?;
    Ok(icon)
}

fn send_profile(profile: Profile) {
    println!("Creating runtime to send profile: {:?}", profile);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let socket_path = PathBuf::from(SOCKET_PATH);
            let setting = ColorSetting::Profile(profile);
            println!("Sending profile command to daemon at {}", socket_path.display());
            if let Err(e) = send_command(socket_path, Command::SetProfile(setting)).await {
                eprintln!("Failed to send profile command: {}", e);
            } else {
                println!("Successfully sent profile command");
            }
        });
}

async fn send_command(socket_path: PathBuf, command: Command) -> Result<()> {
    use tokio::io::AsyncWriteExt;
    let mut stream = tokio::net::UnixStream::connect(socket_path).await?;
    let command_json = serde_json::to_vec(&command)?;
    stream.write_all(&command_json).await?;
    Ok(())
} 