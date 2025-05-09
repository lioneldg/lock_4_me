import { create } from 'zustand';
import { DiscoveredDevice } from '../types';

interface BluetoothStore {
  events: Map<string, DiscoveredDevice>;
  addEvent: (event: DiscoveredDevice) => void;
  clearEvents: () => void;
}

export const useBluetoothStore = create<BluetoothStore>((set) => ({
  events: new Map(),
  addEvent: (event) =>
    set((state) => {
      const newMap = new Map(state.events);
      newMap.set(event.id, event);
      return { events: newMap };
    }),
  clearEvents: () => set({ events: new Map() })
}));
