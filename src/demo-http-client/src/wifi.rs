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
use embedded_svc::ipv4::{self, ClientSettings};

pub struct MyWifi {
    esp_wifi: EspWifi,
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
    pub ip_settings: ClientSettings,
}

pub fn my_wifi(ssid: &str, pass: &str) -> Result<Box<MyWifi>> {
    let netif_stack: Arc<EspNetifStack> = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack: Arc<EspSysLoopStack> = Arc::new(EspSysLoopStack::new()?);
    let default_nvs: Arc<EspDefaultNvs> = Arc::new(EspDefaultNvs::new()?);
    
    let mut esp_wifi = EspWifi::new(netif_stack.clone(), sys_loop_stack.clone(), default_nvs.clone())?;

    info!("Scannning for access points...");
    let ap_infos = esp_wifi.scan()?;
    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            ssid, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    // sets up client and access point
    esp_wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: ssid.into(),
            password: pass.into(),
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

    esp_wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = esp_wifi.get_status();
    //let ip_settings: ClientSettings;
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        info!("Wifi connected");
        Ok(Box::new(MyWifi { esp_wifi, netif_stack, sys_loop_stack, default_nvs, ip_settings }))
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
        //Err(())
    }

    //Ok(Box::new(MyWifi { esp_wifi, netif_stack, sys_loop_stack, default_nvs, ip_settings }))

}
