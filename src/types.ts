export type Settings = {
  target_uuid: string;
  rssi_delta_max: number;
  theme: string;
  language: string;
};

export interface DiscoveredDevice {
  event_type: string;
  local_name: string;
  id: string;
  rssi: number;
  diff_rssi: number;
}

export type Colors = {
  backgroundColor: string;
  backgroundTextColor: string;
  secondaryBackgroundColor: string;
  secondaryBackgroundTextColor: string;
  accentColor: string;
  disabledColor: string;
  borderColor: string;
};

export type Theme = 'light' | 'dark';
