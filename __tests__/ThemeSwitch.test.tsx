import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import ThemeSwitch from '../src/components/ThemeSwitch/ThemeSwitch';

// Mock the theme context
const mockSetTheme = jest.fn();
const mockSetSettings = jest.fn();

const mockThemeContext = {
  theme: 'dark',
  setTheme: mockSetTheme,
  colors: {
    accentColor: '#007acc',
    backgroundColor: '#1a1a1a',
    secondaryBackgroundColor: '#2a2a2a'
  }
};

jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => mockThemeContext
}));

jest.mock('../src/store/settingsStore', () => ({
  useSettingsStore: () => ({
    setSettings: mockSetSettings
  })
}));

// Mock the Icon component
jest.mock('../src/components/Icon/Icon', () => ({
  __esModule: true,
  default: ({ type, size, color }: any) => (
    <svg 
      data-testid="theme-icon" 
      role="presentation"
      style={{ width: '1.5rem', height: '1.5rem' }}
      viewBox="0 0 24 24"
    >
      <path 
        d="M17.75,4.09L15.22,6.03L16.13,9.09L13.5,7.28L10.87,9.09L11.78,6.03L9.25,4.09L12.44,4L13.5,1L14.56,4L17.75,4.09M21.25,11L19.61,12.25L20.2,14.23L18.5,13.06L16.8,14.23L17.39,12.25L15.75,11L17.81,10.95L18.5,9L19.19,10.95L21.25,11M18.97,15.95C19.8,15.87 20.69,17.05 20.16,17.8C19.84,18.25 19.5,18.67 19.08,19.07C15.17,23 8.84,23 4.94,19.07C1.03,15.17 1.03,8.83 4.94,4.93C5.34,4.53 5.76,4.17 6.21,3.85C6.96,3.32 8.14,4.21 8.06,5.04C7.79,7.9 8.75,10.87 10.95,13.06C13.14,15.26 16.1,16.22 18.97,15.95M17.33,17.97C14.5,17.81 11.7,16.64 9.53,14.5C7.36,12.31 6.2,9.5 6.04,6.68C3.23,9.82 3.34,14.64 6.35,17.66C9.37,20.67 14.19,20.78 17.33,17.97Z" 
        style={{ fill: color }}
      />
    </svg>
  ),
  ICON_TYPE: {
    LIGHT_MODE: 'light_mode',
    DARK_MODE: 'dark_mode'
  }
}));

describe('ThemeSwitch Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    // Reset to dark theme
    mockThemeContext.theme = 'dark';
  });

  it('should render theme switch with icon', () => {
    render(<ThemeSwitch />);
    
    expect(screen.getByTestId('theme-icon')).toBeInTheDocument();
  });

  it('should render with clickable container', () => {
    render(<ThemeSwitch />);
    
    const switchContainer = screen.getByTestId('theme-icon').closest('div');
    expect(switchContainer).toBeInTheDocument();
  });

  it('should call setTheme and setSettings when clicked', () => {
    render(<ThemeSwitch />);
    
    const switchElement = screen.getByTestId('theme-icon').closest('div');
    fireEvent.click(switchElement!);
    
    expect(mockSetTheme).toHaveBeenCalledWith('light');
    expect(mockSetSettings).toHaveBeenCalledWith({ theme: 'light' });
  });

  it('should switch to dark theme when currently light', () => {
    // Change theme to light
    mockThemeContext.theme = 'light';
    
    render(<ThemeSwitch />);
    
    const switchElement = screen.getByTestId('theme-icon').closest('div');
    fireEvent.click(switchElement!);
    
    expect(mockSetTheme).toHaveBeenCalledWith('dark');
    expect(mockSetSettings).toHaveBeenCalledWith({ theme: 'dark' });
  });
});