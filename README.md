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
# new project
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo

# docs
cargo doc --open

# flash and monitor
# requies espflash 1.5.1 (https://github.com/esp-rs/espflash/issues/209)
espflash --monitor COM3 .\target\xtensa-esp32-espidf\debug\demo-http-client
```

## Resources

-[ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-wroom-32e_esp32-wroom-32ue_datasheet_en.pdf)
- [Blog post on LoRa build](https://randomnerdtutorials.com/esp32-lora-rfm95-transceiver-arduino-ide/)


## Environment

```bash

```