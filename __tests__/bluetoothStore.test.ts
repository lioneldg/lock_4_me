import { useBluetoothStore } from '../src/store/bluetoothStore';
import { DiscoveredDevice } from '../src/types';

describe('BluetoothStore', () => {
  beforeEach(() => {
    // Reset the store before each test
    useBluetoothStore.getState().clearEvents();
  });

  it('should have initial state', () => {
    const state = useBluetoothStore.getState();
    expect(state.events).toEqual(new Map());
    expect(state.events.size).toBe(0);
  });

  it('should add a bluetooth event', () => {
    const mockDevice: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    const { addEvent } = useBluetoothStore.getState();
    addEvent(mockDevice);

    const state = useBluetoothStore.getState();
    expect(state.events.size).toBe(1);
    expect(state.events.get('device1')).toEqual(mockDevice);
  });

  it('should update existing device when adding with same id', () => {
    const mockDevice1: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    const mockDevice2: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device Updated',
      rssi: -45,
      diff_rssi: 10
    };

    const { addEvent } = useBluetoothStore.getState();
    addEvent(mockDevice1);
    addEvent(mockDevice2);

    const state = useBluetoothStore.getState();
    expect(state.events.size).toBe(1);
    expect(state.events.get('device1')).toEqual(mockDevice2);
  });

  it('should add multiple different devices', () => {
    const mockDevice1: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device 1',
      rssi: -50,
      diff_rssi: 5
    };

    const mockDevice2: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device2',
      local_name: 'Test Device 2',
      rssi: -60,
      diff_rssi: 8
    };

    const { addEvent } = useBluetoothStore.getState();
    addEvent(mockDevice1);
    addEvent(mockDevice2);

    const state = useBluetoothStore.getState();
    expect(state.events.size).toBe(2);
    expect(state.events.get('device1')).toEqual(mockDevice1);
    expect(state.events.get('device2')).toEqual(mockDevice2);
  });

  it('should clear all events', () => {
    const mockDevice: DiscoveredDevice = {
      event_type: 'bluetooth-event',
      id: 'device1',
      local_name: 'Test Device',
      rssi: -50,
      diff_rssi: 5
    };

    const { addEvent, clearEvents } = useBluetoothStore.getState();
    addEvent(mockDevice);
    
    expect(useBluetoothStore.getState().events.size).toBe(1);
    
    clearEvents();
    
    expect(useBluetoothStore.getState().events.size).toBe(0);
    expect(useBluetoothStore.getState().events).toEqual(new Map());
  });
});