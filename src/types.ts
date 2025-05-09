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
  text: string;
  background: string;
  primary: string;
  onPrimary: string;
  secondary: string;
  onSecondary: string;
  error: string;
  onError: string;
  warning: string;
  info: string;
  success: string;
  surface: string;
  onSurface: string;
  accentColor: string;
};

export type Theme = 'light' | 'dark';
