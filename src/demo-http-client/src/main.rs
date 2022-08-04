mod wifi;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use crate::wifi::my_wifi;
use esp_idf_logger;
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Headers, Status,
    },
    io::Read,
};
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};
use esp_idf_svc::netif::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::ping;
use embedded_svc::ping::Ping;
use anyhow::bail;
use anyhow::Result;
use embedded_svc::wifi::*;
use log::*;
use std::sync::Arc;
use std::time::*;
use embedded_svc::ipv4;

const SSID: &str = "Asus WiFi";
const PASS: &str = "";

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    esp_idf_logger::init().unwrap();

    #[allow(unused)]
    let netif_stack = Arc::new(EspNetifStack::new()?);
    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new()?);

    let mut wifi = my_wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    //get("http://neverssl.com");

    println!("Fin");
    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    // 1. create a new EspHttpClient with SSL certificates enabled
    let mut client = EspHttpClient::new_default()?;
    // let mut client = EspHttpClient::new(&EspHttpClientConfiguration {
    //     //use_global_ca_store: true,
    //     //crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

    //     ..Default::default()
    // })?;

    // 2. open a GET request to `url`
    let request = client.get(url.as_ref())?;

    // 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);
    let writer = request.into_writer(0)?;

    // 4. Turn the writer into a response and check its status. 
    // Successful http status codes are in the 200..=299 range.

    let response = writer.submit()?;
    let status = response.status();
    let mut total_size = 0;

    println!("response code: {}\n", status);

    Ok(())
}

// fn wifi(
//     netif_stack: Arc<EspNetifStack>,
//     sys_loop_stack: Arc<EspSysLoopStack>,
//     default_nvs: Arc<EspDefaultNvs>,
// ) -> Result<Box<EspWifi>> {
//     let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);
//
//     info!("Wifi created, about to scan");
//
//     let ap_infos = wifi.scan()?;
//
//     let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);
//
//     let channel = if let Some(ours) = ours {
//         info!(
//             "Found configured access point {} on channel {}",
//             SSID, ours.channel
//         );
//         Some(ours.channel)
//     } else {
//         info!(
//             "Configured access point {} not found during scanning, will go with unknown channel",
//             SSID
//         );
//         None
//     };
//
//     wifi.set_configuration(&Configuration::Mixed(
//         ClientConfiguration {
//             ssid: SSID.into(),
//             password: PASS.into(),
//             channel,
//             ..Default::default()
//         },
//         AccessPointConfiguration {
//             ssid: "aptest".into(),
//             channel: channel.unwrap_or(1),
//             ..Default::default()
//         },
//     ))?;
//
//     info!("Wifi configuration set, about to get status");
//
//     wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
//         .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;
//
//     let status = wifi.get_status();
//
//     if let Status(
//         ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
//         ApStatus::Started(ApIpStatus::Done),
//     ) = status
//     {
//         info!("Wifi connected");
//
//         ping(&ip_settings)?;
//     } else {
//         bail!("Unexpected Wifi status: {:?}", status);
//     }
//
//     Ok(wifi)
// }

// fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
//     info!("About to do some pings for {:?}", ip_settings);
//
//     let ping_summary =
//         ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
//     if ping_summary.transmitted != ping_summary.received {
//         bail!(
//             "Pinging gateway {} resulted in timeouts",
//             ip_settings.subnet.gateway
//         );
//     }
//
//     info!("Pinging done");
//
//     Ok(())
// }
