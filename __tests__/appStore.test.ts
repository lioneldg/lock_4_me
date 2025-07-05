import { useAppStore } from '../src/store/appStore';

describe('AppStore', () => {
  beforeEach(() => {
    // Reset the store before each test
    useAppStore.getState().setIsLoading(false);
    useAppStore.getState().setIsDiscoveryMode(false);
  });

  it('should have initial state', () => {
    const state = useAppStore.getState();
    expect(state.isLoading).toBe(false);
    expect(state.isDiscoveryMode).toBe(false);
  });

  it('should update isLoading state', () => {
    const { setIsLoading } = useAppStore.getState();
    
    setIsLoading(true);
    expect(useAppStore.getState().isLoading).toBe(true);
    
    setIsLoading(false);
    expect(useAppStore.getState().isLoading).toBe(false);
  });

  it('should update isDiscoveryMode state', () => {
    const { setIsDiscoveryMode } = useAppStore.getState();
    
    setIsDiscoveryMode(true);
    expect(useAppStore.getState().isDiscoveryMode).toBe(true);
    
    setIsDiscoveryMode(false);
    expect(useAppStore.getState().isDiscoveryMode).toBe(false);
  });

  it('should handle multiple state updates', () => {
    const { setIsLoading, setIsDiscoveryMode } = useAppStore.getState();
    
    setIsLoading(true);
    setIsDiscoveryMode(true);
    
    const state = useAppStore.getState();
    expect(state.isLoading).toBe(true);
    expect(state.isDiscoveryMode).toBe(true);
  });
});