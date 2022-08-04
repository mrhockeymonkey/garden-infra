// based on https://github.com/ferrous-systems/espressif-trainings/blob/main/common/lib/esp32-c3-dkc02-bsc/src/wifi.rs



use anyhow::bail;
use embedded_svc::wifi::{
    self, AuthMethod, ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus,
    Wifi as _,
};
use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};
use log::info;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};
use esp_idf_svc::netif::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::ping;
use embedded_svc::ping::Ping;
use anyhow::Result;
use embedded_svc::wifi::*;
use log::*;
use std::sync::Arc;
use std::time::*;
use embedded_svc::ipv4;

const SSID: &str = "Asus WiFi";
const PASS: &str = "";

#[allow(unused)]
pub struct Wifi {
    esp_wifi: EspWifi,
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
}

pub fn my_wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        info!("Wifi connected");

        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

// pub fn wifi(ssid: &str, psk: &str) -> anyhow::Result<Wifi> {
//     let mut auth_method = AuthMethod::WPA2Personal;
//     if ssid.len() == 0 {
//         anyhow::bail!("missing WiFi name")
//     }
//     if psk.len() == 0 {
//         auth_method = AuthMethod::None;
//         info!("Wifi password is empty");
//     }
//     let netif_stack = Arc::new(EspNetifStack::new()?);
//     let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
//     let default_nvs = Arc::new(EspDefaultNvs::new()?);
//     let mut wifi = EspWifi::new(
//         netif_stack.clone(),
//         sys_loop_stack.clone(),
//         default_nvs.clone(),
//     )?;
//
//     info!("Searching for Wifi network {}", ssid);
//
//     let ap_infos = wifi.scan()?;
//
//     let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);
//
//     let channel = if let Some(ours) = ours {
//         info!(
//             "Found configured access point {} on channel {}",
//             ssid, ours.channel
//         );
//         Some(ours.channel)
//     } else {
//         info!(
//             "Configured access point {} not found during scanning, will go with unknown channel",
//             ssid
//         );
//         None
//     };
//
//     info!("setting Wifi configuration");
//     wifi.set_configuration(&wifi::Configuration::Client(ClientConfiguration {
//         ssid: ssid.into(),
//         password: psk.into(),
//         channel,
//         auth_method: auth_method,
//         ..Default::default()
//     }))?;
//
//     info!("getting Wifi status");
//
//     let status = wifi.get_status();
//
//     if let wifi::Status(
//         ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_))),
//         _,
//     ) = status
//     {
//         info!("Wifi connected!");
//     } else {
//         bail!("Unexpected Wifi status: {:?}", status);
//     }
//
//     let wifi = Wifi {
//         esp_wifi: wifi,
//         netif_stack,
//         sys_loop_stack,
//         default_nvs,
//     };
//
//     Ok(wifi)
// }

fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    info!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    info!("Pinging done");

    Ok(())
}