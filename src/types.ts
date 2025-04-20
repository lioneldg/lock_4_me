export type Settings = {
  target_uuid: string;
  rssi_delta_max: number;
};

export interface DiscoveredDevice {
  event_type: string;
  local_name: string;
  id: string;
  rssi: number;
  diff_rssi: number;
}
