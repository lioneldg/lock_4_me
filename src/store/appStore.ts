import { create } from 'zustand';

interface AppStore {
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
  isDiscoveryMode: boolean;
  setIsDiscoveryMode: (discoveryMode: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  isLoading: false,
  isDiscoveryMode: false,
  setIsLoading: (loading) => set({ isLoading: loading }),
  setIsDiscoveryMode: (discoveryMode) => set({ isDiscoveryMode: discoveryMode })
}));
