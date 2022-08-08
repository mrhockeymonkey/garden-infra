# garden-infra
My bespoke gardening infrastructure

## Project Goals

- Design and Implement a battery/solar powered garden sensor array
- Collect and send statistics to the cloud (soil moisrure, battery level)
- Create simple companion app

## Learning Objectives and Milestones

- [] Understand Rust embeded development
- [] Build working sensor system as POC
- [] Demo project sending data to Azure
- [] Upgrade sensor system to run on battery
- [] Upgrade system to utilize LoRa instead of WiFi


## Notes

```powershell
# connect esp board to WSL
usbipd wsl list
usbipd wsl attach --hardware-id 10c4:ea60
```

```bash
# new project
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo

# docs
cargo doc --open

# flash and monitor
sudo chown scott /dev/ttyUSB0
espflash --monitor /dev/ttyUSB0 ./target/xtensa-esp32-espidf/debug/demo-http-client
```

## Resources

-[ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-wroom-32e_esp32-wroom-32ue_datasheet_en.pdf)
- [Blog post on LoRa build](https://randomnerdtutorials.com/esp32-lora-rfm95-transceiver-arduino-ide/)


## Environment Setup

```bash
# install basics
sudo apt update
sudo apt install python3-venv python3-pip llvm-dev libclang-dev clang

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly

# install cmake
sudo apt install software-properties-common lsb-release
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | sudo tee /etc/apt/trusted.gpg.d/kitware.gpg >/dev/null
sudo apt-add-repository "deb https://apt.kitware.com/ubuntu/ $(lsb_release -cs) main"
sudo apt update && sudo apt install kitware-archive-keyring
sudo apt update && sudo apt install cmake

# install esp deps
sudo apt install ninja-build flex bison gperf ccache libffi-dev libssl-dev dfu-util libusb-1.0-0 zip

cargo install ldproxy
sudo apt install pkg-config libudev-dev
cargo install espflash
cargo install espmonitor

# install esp toolchain
curl -LO https://raw.githubusercontent.com/esp-rs/rust-build/main/install-rust-toolchain.sh
chmod +x install-rust-toolchain.sh
./install-rust-toolchain.sh --esp-idf-version release/v4.4 --build-target esp32
# or --installation-mode reinstall

# enable environment (or put this in .bashrc)
export LIBCLANG_PATH="/home/scott/.espressif/tools/xtensa-esp32-elf-clang/esp-14.0.0-20220415-x86_64-unknown-linux-gnu/lib/"
export IDF_TOOLS_PATH=/home/scott/.espressif
source /home/scott/.espressif/frameworks/esp-idf-release-v4.4/export.sh
```

