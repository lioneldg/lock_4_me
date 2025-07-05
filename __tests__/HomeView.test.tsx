import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { BrowserRouter } from 'react-router';
import '@testing-library/jest-dom';
import HomeView from '../src/views/HomeView';
import { DiscoveredDevice } from '../src/types';

// Mock the router
const MockRouter = ({ children }: { children: React.ReactNode }) => (
  <BrowserRouter>{children}</BrowserRouter>
);

// Mock the stores
const mockBluetoothStore = {
  events: new Map<string, DiscoveredDevice>(),
  clearEvents: jest.fn(),
  addEvent: jest.fn()
};

const mockSettingsStore = {
  settings: {
    target_uuid: 'test-uuid-123',
    rssi_delta_max: 15,
    theme: 'dark' as const,
    language: 'en' as const
  },
  setSettings: jest.fn()
};

const mockAppStore = {
  isDiscoveryMode: false,
  setIsDiscoveryMode: jest.fn(),
  isLoading: false,
  setIsLoading: jest.fn()
};

jest.mock('../src/store/bluetoothStore', () => ({
  useBluetoothStore: () => mockBluetoothStore
}));

jest.mock('../src/store/settingsStore', () => ({
  useSettingsStore: () => mockSettingsStore
}));

jest.mock('../src/store/appStore', () => ({
  useAppStore: () => mockAppStore
}));

// Mock react-router
const mockNavigate = jest.fn();
jest.mock('react-router', () => ({
  ...jest.requireActual('react-router'),
  useNavigate: () => mockNavigate
}));

// Mock translations
jest.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, options?: any) => {
      const translations: { [key: string]: string } = {
        'home.title': 'Lock 4 Me',
        'home.discoveryMode': 'Discovery Mode - Select a device',
        'home.targetMode': `Target Mode - Tracking: ${options?.uuid || 'test-uuid-123'}`
      };
      return translations[key] || key;
    }
  })
}));

// Mock components
jest.mock('../src/components/Icon', () => ({
  __esModule: true,
  default: ({ type }: { type: string }) => (
    <div data-testid="icon" data-type={type}>
      Icon
    </div>
  )
}));

jest.mock('../src/components/FormattedText', () => ({
  __esModule: true,
  default: ({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) => (
    <span data-testid="formatted-text" style={style}>
      {children}
    </span>
  )
}));

describe('HomeView Integration Tests', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockBluetoothStore.events.clear();
    mockAppStore.isDiscoveryMode = false;
  });

  it('should render home view with title and settings button', () => {
    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    expect(screen.getByText('Lock 4 Me')).toBeInTheDocument();
    expect(screen.getByTestId('icon')).toBeInTheDocument();
  });

  it('should display target mode when not in discovery mode', () => {
    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    expect(screen.getByText(/Target Mode - Tracking: test-uuid-123/)).toBeInTheDocument();
  });

  it('should display discovery mode when in discovery mode', () => {
    mockAppStore.isDiscoveryMode = true;
    
    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    expect(screen.getByText('Discovery Mode - Select a device')).toBeInTheDocument();
  });

  it('should render discovered devices', () => {
    const mockDevice: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    mockBluetoothStore.events.set('device1', mockDevice);

    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    expect(screen.getByText(/Test Device/)).toBeInTheDocument();
    expect(screen.getByText(/RSSI: -50 dBm/)).toBeInTheDocument();
    expect(screen.getByText(/Δ: 5 dBm/)).toBeInTheDocument();
  });

  it('should navigate to settings when settings button is clicked', () => {
    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    const settingsButton = screen.getByTestId('icon').parentElement;
    fireEvent.click(settingsButton!);

    expect(mockNavigate).toHaveBeenCalledWith('/settings');
  });

  it('should select device in discovery mode', () => {
    mockAppStore.isDiscoveryMode = true;
    
    const mockDevice: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    mockBluetoothStore.events.set('device1', mockDevice);

    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    const deviceElement = screen.getByText(/Test Device/).parentElement;
    fireEvent.click(deviceElement!);

    expect(mockBluetoothStore.clearEvents).toHaveBeenCalled();
    expect(mockSettingsStore.setSettings).toHaveBeenCalledWith({ target_uuid: 'device1' });
    expect(mockAppStore.setIsDiscoveryMode).toHaveBeenCalledWith(false);
  });

  it('should not select device when not in discovery mode', () => {
    mockAppStore.isDiscoveryMode = false;
    
    const mockDevice: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    mockBluetoothStore.events.set('device1', mockDevice);

    render(
      <MockRouter>
        <HomeView />
      </MockRouter>
    );

    const deviceElement = screen.getByText(/Test Device/).parentElement;
    fireEvent.click(deviceElement!);

    expect(mockBluetoothStore.clearEvents).not.toHaveBeenCalled();
    expect(mockSettingsStore.setSettings).not.toHaveBeenCalled();
    expect(mockAppStore.setIsDiscoveryMode).not.toHaveBeenCalled();
  });
});