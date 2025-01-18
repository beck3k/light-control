This is the app I use to control lighting around my computer. I got a new computer and needed to move the app, so I decided to put it on github. 
I was given a Pimoroni Mote LED gizmo, so that's the primary target, but it can definately be extended to other devices. I have the Mote strips attached around my monitors, and the app currently has two profiles, a white which I use for meetings, and a red which I use mostly at night to offset the blue light from my monitors.

To build you will need Python and Rust. The python is because of the mote library. Depending on your system, you may need to install Xcode or setup a virtual environment. 

The program is constructed in two parts:
1. The daemon, which runs in the background and manages the lighting. You can also run it as a cli to send commands to the daemon.
2. The tray, which provides a menu for switching profiles, through an icon in the system tray.

To bundle the app for MacOS, run the build.sh script. It will build the rust binaries, and create a virtualenv which is bundled with the app. The app starts both the daemon and the tray application.
You will find the app in the target/release directory.
