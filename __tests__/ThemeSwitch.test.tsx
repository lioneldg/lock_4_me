import React from 'react';
import { render, fireEvent, screen } from '@testing-library/react';
import { describe, it, expect, jest } from '@jest/globals';
import '@testing-library/jest-dom';
import ThemeSwitch from '../src/components/ThemeSwitch';

// Mock the hooks and components used
jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => ({
    theme: 'dark',
    setTheme: jest.fn(),
    colors: {
      accentColor: '#007acc',
      secondaryBackgroundColor: '#2a2a2a'
    }
  })
}));

jest.mock('../src/store/settingsStore', () => ({
  useSettingsStore: () => ({
    setSettings: jest.fn()
  })
}));

jest.mock('../src/components/Icon', () => ({
  __esModule: true,
  default: ({ type }: { type: string }) => (
    <div data-testid="icon" data-type={type}>
      Icon
    </div>
  )
}));

jest.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: { [key: string]: string } = {
        'settings.theme': 'Theme',
        'settings.dark_mode': 'Dark Mode',
        'settings.light_mode': 'Light Mode'
      };
      return translations[key] || key;
    }
  })
}));

describe('ThemeSwitch Component', () => {
  it('should render theme switch with label', () => {
    render(<ThemeSwitch />);
    
    expect(screen.getByText('Theme')).toBeInTheDocument();
    expect(screen.getByText('Dark Mode')).toBeInTheDocument();
  });

  it('should render with correct initial state for dark theme', () => {
    render(<ThemeSwitch />);
    
    const switchElement = screen.getByRole('checkbox');
    expect(switchElement).toBeChecked();
  });

  it('should call setTheme and setSettings when toggled', () => {
    const mockSetTheme = jest.fn();
    const mockSetSettings = jest.fn();
    
    jest.doMock('../src/hooks/ThemeContext', () => ({
      useTheme: () => ({
        theme: 'dark',
        setTheme: mockSetTheme,
        colors: {
          accentColor: '#007acc',
          secondaryBackgroundColor: '#2a2a2a'
        }
      })
    }));
    
    jest.doMock('../src/store/settingsStore', () => ({
      useSettingsStore: () => ({
        setSettings: mockSetSettings
      })
    }));
    
    render(<ThemeSwitch />);
    
    const switchElement = screen.getByRole('checkbox');
    fireEvent.click(switchElement);
    
    expect(mockSetTheme).toHaveBeenCalledWith('light');
    expect(mockSetSettings).toHaveBeenCalledWith({ theme: 'light' });
  });
});