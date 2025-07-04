export type Language = 'en' | 'en-GB';

export type Theme = 'light' | 'dark';

export type Settings = {
  target_uuid: string;
  rssi_delta_max: number;
  theme: Theme;
  language: Language;
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
