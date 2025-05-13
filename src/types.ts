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
  secondaryBackgroundSelectorColor: string;
  accentColor: string;
  accentTextColor: string;
  accentHoverColor: string;
  brandColor: string;
  brandHoverColor: string;
  brandTextColor: string;
  brandHoverTextColor: string;
  selectorColor: string;
  disabledColor: string;
  separatorColor: string;
  borderColor: string;
  redColor: string;
  buttonPrimaryColor: string;
  buttonRedColor: string;
  buttonGreyColor: string;
};

export type Theme = 'light' | 'dark';
