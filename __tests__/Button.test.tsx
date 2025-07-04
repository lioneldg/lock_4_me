import React from 'react';
import { render, fireEvent, screen } from '@testing-library/react';
import { describe, it, expect, jest } from '@jest/globals';
import '@testing-library/jest-dom';
import { Button } from '../src/components/Button';
import { ICON_TYPE } from '../src/components/Icon/Icon';

// Mock the hooks and components used in the Button
jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => ({
    colors: {
      secondaryBackgroundColor: '#2a2a2a',
      accentColor: '#007acc',
      disabledColor: '#666666',
      backgroundTextColor: '#ffffff'
    }
  })
}));

jest.mock('../src/components/Icon', () => ({
  __esModule: true,
  default: ({ type, size, color }: { type: string; size: number; color: string }) => (
    <div data-testid="icon" data-type={type} data-size={size} data-color={color}>
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

describe('Button Component', () => {
  it('should render button with text only', () => {
    const onPress = jest.fn();
    render(<Button text="Click me" onPress={onPress} />);

    expect(screen.getByTestId('formatted-text')).toBeInTheDocument();
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('should render button with icon only', () => {
    const onPress = jest.fn();
    render(<Button icon={ICON_TYPE.SETTINGS} onPress={onPress} />);

    expect(screen.getByTestId('icon')).toBeInTheDocument();
    expect(screen.getByTestId('icon')).toHaveAttribute('data-type', ICON_TYPE.SETTINGS);
  });

  it('should render button with both icon and text', () => {
    const onPress = jest.fn();
    render(<Button icon={ICON_TYPE.SETTINGS} text="Settings" onPress={onPress} />);

    expect(screen.getByTestId('icon')).toBeInTheDocument();
    expect(screen.getByTestId('formatted-text')).toBeInTheDocument();
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });

  it('should call onPress when clicked', () => {
    const onPress = jest.fn();
    render(<Button text="Click me" onPress={onPress} />);

    fireEvent.click(screen.getByText('Click me'));
    expect(onPress).toHaveBeenCalledTimes(1);
  });

  it('should not call onPress when disabled', () => {
    const onPress = jest.fn();
    render(<Button text="Disabled" onPress={onPress} isDisabled={true} />);

    fireEvent.click(screen.getByText('Disabled'));
    expect(onPress).not.toHaveBeenCalled();
  });

  it('should apply disabled styles when disabled', () => {
    const onPress = jest.fn();
    render(<Button text="Disabled" onPress={onPress} isDisabled={true} />);

    const button = screen.getByText('Disabled').parentElement;
    expect(button).toHaveStyle('cursor: default');
  });

  it('should apply reverse color scheme when reverseColor is true', () => {
    const onPress = jest.fn();
    render(<Button text="Reverse" onPress={onPress} reverseColor={true} />);

    const formattedText = screen.getByTestId('formatted-text');
    expect(formattedText).toHaveStyle('color: #2a2a2a');
  });

  it('should apply custom dimensions', () => {
    const onPress = jest.fn();
    render(<Button text="Custom" onPress={onPress} width={10} height={3} />);

    const button = screen.getByText('Custom').parentElement;
    expect(button).toHaveStyle('width: 10rem');
    expect(button).toHaveStyle('height: 3rem');
  });

  it('should apply custom colors', () => {
    const onPress = jest.fn();
    render(
      <Button
        text="Custom Colors"
        onPress={onPress}
        textColor="#ff0000"
        backgroundColor="#00ff00"
      />
    );

    const button = screen.getByText('Custom Colors').parentElement;
    const formattedText = screen.getByTestId('formatted-text');
    
    expect(button).toHaveStyle('background-color: #00ff00');
    expect(formattedText).toHaveStyle('color: #ff0000');
  });

  it('should apply noBorder and noPadding styles', () => {
    const onPress = jest.fn();
    render(<Button text="No Border" onPress={onPress} noBorder={true} noPadding={true} />);

    const button = screen.getByText('No Border').parentElement;
    expect(button).toHaveStyle('border: 0px solid #ffffff');
    expect(button).toHaveStyle('padding: 0rem');
  });

  it('should apply custom border radius', () => {
    const onPress = jest.fn();
    render(<Button text="Rounded" onPress={onPress} borderRadius={20} />);

    const button = screen.getByText('Rounded').parentElement;
    expect(button).toHaveStyle('border-radius: 20px');
  });
});