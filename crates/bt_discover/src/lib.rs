use async_stream::stream;
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::Stream;
use futures::stream::StreamExt;
use std::error::Error;
use std::io;
use std::pin::Pin;
use uuid::Uuid;

// return bluetooth adapter
pub async fn init_bluetooth() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = match adapters.first() {
        Some(adapter) => adapter.clone(),
        None => {
            eprintln!("No Bluetooth adapters found");
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "No Bluetooth adapters found",
            )));
        }
    };
    Ok(adapter)
}

async fn get_bt_events_listener(
    adapter: &Adapter,
) -> Result<impl Stream<Item = CentralEvent> + Send, Box<dyn Error>> {
    adapter.start_scan(ScanFilter::default()).await?;
    let events = adapter.events().await?;
    Ok(events)
}

/// Struct representing a discovered Bluetooth device
pub struct DiscoveredDevice {
    pub id: String,
    pub local_name: Option<String>,
    pub rssi: Option<i16>,
    pub event_type: String,
}

/// Returns a stream of discovered or updated Bluetooth devices matching the optional target_uuid
pub fn bluetooth_devices_stream(
    adapter: Adapter,
    target_uuid: Option<Uuid>,
) -> Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>> {
    Box::pin(stream! {
        let mut events = match get_bt_events_listener(&adapter).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to get Bluetooth events: {}", e);
                return;
            }
        };
        while let Some(event) = events.next().await {
            match &event {
                CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
                    if let Some(target) = target_uuid {
                        if id.to_string() != target.to_string() {
                            continue;
                        }
                    }
                    if let Ok(peripheral) = adapter.peripheral(id).await {
                        if let Ok(properties) = peripheral.properties().await {
                            let local_name = properties.as_ref().and_then(|p| p.local_name.clone());
                            let rssi = properties.as_ref().and_then(|p| p.rssi);
                            let event_type = match &event {
                                CentralEvent::DeviceDiscovered(_) => "Discovered device",
                                _ => "Device updated",
                            };
                            yield DiscoveredDevice {
                                id: id.to_string(),
                                local_name,
                                rssi,
                                event_type: event_type.to_string(),
                            };
                        }
                    }
                }
                _ => {}
            }
        }
    })
}

/// Returns a stream of discovered or updated Bluetooth devices matching the optional target_uuid, handling Bluetooth initialization internally
pub async fn discover_bluetooth_devices(
    target_uuid: Option<Uuid>,
) -> Result<Pin<Box<dyn Stream<Item = DiscoveredDevice> + Send>>, Box<dyn std::error::Error>> {
    let adapter = init_bluetooth().await?;
    Ok(bluetooth_devices_stream(adapter, target_uuid))
}
