use objc2_core_location::CLLocationManager;
use objc2_core_wlan::CWWiFiClient;
//use objc2_foundation::{ns_string, NSString};
use tracing::info;

struct Bssid([u8; 6]);
/// Needs access to location to get BSSID since there are privacy concerns
/// maybe could notify server that it doesn't have permission so that IT can see if a laptop
/// doesn't have permissions?
pub fn get_bssid() {
    unsafe {
        let client = CWWiFiClient::sharedWiFiClient();
        if let Some(interface) = client.interface() {
            println!("{:?}", interface.noiseMeasurement());
        }
    }
}

pub fn request_location() {
    unsafe {
        if CLLocationManager::locationServicesEnabled_class() {
            info!("location services are enabled")
        } else {
            let loc_manager = CLLocationManager::new();
            loc_manager.requestAlwaysAuthorization();
        }
    }
}
