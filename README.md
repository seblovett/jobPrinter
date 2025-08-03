# jobPrinter
A Rust Application to print jobs out to a Thermal Printer
This is heavily inspired by [this post](https://www.laurieherault.com/articles/a-thermal-receipt-printer-cured-my-procrastination) 

My general idea builds on this, but for the routine tasks. 
I bought a cheap [ESC/POS printer](https://www.amazon.co.uk/Thermal-Receipt-Ethernet-Restaurant-Business/dp/B0CM6DHW7R/ref=sr_1_19?crid=20EZSNGJFXCGW&dib=eyJ2IjoiMSJ9.o3fm9-MBbcO4DLSwFI2irzD5-B35HbgUpGa0aOW5WGUZGRhG2wV56EYPd6Xma5--4vxLkAPMKv87JzmzrDnBQU_NSEZtAd-dSgyPmwR-8w_YUlkoGxDDMJjxsGEu24_2NKobcnmuT5NJyGmNMFINJyxK5PgVd8Iji_n6roaFzo799b_4Ewq5olCjUfNDwAveXbJvV2dWG68fGKsIxAXd0dkDDaG4CaDEMMn9e9dy2ad_S_f4G0mABlSbcOIMgLET89VCz3xp5pu898GQrSWRjisIeZZnp0SCTiY5KHdaZr0.34M7ZvbQPcaaO6cEPT-NCR0PAjuhmQCNzXYJqv0KyAU&dib_tag=se&keywords=receipt+printer&qid=1752759570&sprefix=receipt+printer%2Caps%2C95&sr=8-19) to do this (not branded).
It will sit in the kitchen and spit out jobs at specified times, or when sent. 
A RPi will be added to the device to make it fully standalone. 

# Requirements
Using a rough MoSCoW principle. These will undoubtedly change over time.
## Must
- Print routine jobs out at a specific time or interval 
    e.g. Sunday 19:00 - put bins out; Every 5 days, water the plants. 
- Be a standalone system
- Be able to send one off jobs / list of jobs to be printed. 
- Use Rust (I have been learning and want to do a project with it!)

## Should

## Could
- Implement a rewards scheme for completing jobs.
- Have a points tally to keep score
- Power down the printer when not in use

## Won't

# Implementation
--------------
The plan is:
- Host a REST API to submit jobs to (POST)
- On POST, job is stored to a database and printed
- In future, keeps tabs on open jobs and scores. 
- Web interface to show open jobs and a score board


A scheduler will be implemented that keeps jobs with a schedule
It will submit a POST to print the job at the right time.
Schedules should be in CRON format, `0 19 * * 0; Put bins out`

# Windows
- Install Rust.
- The USB drivers by default are not compatible.
- Download Zadig and change to be libusb. 
- Clone the repo 
- `cargo run`

# Linux 
The following commands are needed to run this project (YMMV).
```
#install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#install required packages
sudo apt install build-essential libudev-dev  pkg-config

# add current user to the lp group (for permissions). This could be done with a udev rule
sudo usermod -a -G lp $USER

#clone repo
git clone git@github.com:seblovett/jobPrinter.git
cd jobPrinter

# build and run
cargo run
```

# Cross compiling

The project runs on a Raspberry Pi Zero 2 W. 
Building on this is slow (I never managed it).

The Rust way of cross-compiling seems to be to avoid, at all costs, cross-compiling. 
The below is a summary of a lot of Googling, and to be honest, the solution was begrudgingly obtained from ChatGPT... 

If you are doing something that doesn't require any kernel headers or objects, then it's all straight forward. 

I initially tried to get this working on a RPi Zero W, but [ARMv6 is not supported](https://github.com/cross-rs/cross/wiki/Additional-External-Dependency-Issues).

To "cross" compile, follow these instructions, changing targets where appropriate. 
(The following should work for a RPi Zero 2 W)
```
rustup --target=arm-unknown-linux-gnueabihf
cd cross-armv7-libudev
docker build -t local/cross-armv7-libudev -f Dockerfile.armv7-unknown-linux-gnueabihf .
cd ..
cross build --target=armv7-unknown-linux-gnueabihf

```

Set up a RPi in your favourite way. This is covered in many places, I won't repeat this here. 
You may also need to install `libc6-dev` and `libudev-dev`. 

Copy the executable to the device using your favourite copy protocol. 

Run the executable! 

Running in release doesn't seem to work... need to figure out why.
