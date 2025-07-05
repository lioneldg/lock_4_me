import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import Button from '../src/components/Button/Button';

// Mock components and hooks
jest.mock('../src/components/Icon/Icon', () => ({
  __esModule: true,
  default: ({ type, size, color }: any) => (
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

jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => ({
    colors: {
      accentColor: '#007acc',
      backgroundColor: '#1a1a1a',
      secondaryBackgroundColor: '#2a2a2a',
      backgroundTextColor: '#ffffff',
      disabledColor: '#666666'
    }
  })
}));

describe('Button Component', () => {
  const mockOnPress = jest.fn();

  beforeEach(() => {
    mockOnPress.mockClear();
  });

  it('should render button with text only', () => {
    render(<Button text="Click me" onPress={mockOnPress} />);
    
    expect(screen.getByText('Click me')).toBeInTheDocument();
    expect(screen.queryByTestId('icon')).not.toBeInTheDocument();
  });

  it('should render button with icon only', () => {
    render(<Button icon="test-icon" onPress={mockOnPress} />);
    
    expect(screen.getByTestId('icon')).toBeInTheDocument();
    expect(screen.queryByTestId('formatted-text')).not.toBeInTheDocument();
  });

  it('should render button with both icon and text', () => {
    render(<Button icon="test-icon" text="With Icon" onPress={mockOnPress} />);
    
    expect(screen.getByTestId('icon')).toBeInTheDocument();
    expect(screen.getByText('With Icon')).toBeInTheDocument();
  });

  it('should call onPress when clicked', () => {
    render(<Button text="Click me" onPress={mockOnPress} />);
    
    const button = screen.getByText('Click me').parentElement;
    fireEvent.click(button!);
    
    expect(mockOnPress).toHaveBeenCalledTimes(1);
  });

  it('should not call onPress when disabled', () => {
    render(<Button text="Disabled" onPress={mockOnPress} isDisabled={true} />);
    
    const button = screen.getByText('Disabled').parentElement;
    fireEvent.click(button!);
    
    expect(mockOnPress).not.toHaveBeenCalled();
  });

  it('should render disabled button properly', () => {
    render(<Button text="Disabled" onPress={mockOnPress} isDisabled={true} />);
    
    const button = screen.getByText('Disabled').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should render button with reverse color scheme', () => {
    render(<Button text="Reverse" onPress={mockOnPress} reverseColor={true} />);
    
    const button = screen.getByText('Reverse').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should render button with custom dimensions', () => {
    render(<Button text="Custom" onPress={mockOnPress} width={10} height={3} />);
    
    const button = screen.getByText('Custom').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should render button with custom background and text colors', () => {
    render(
      <Button 
        text="Custom Colors" 
        onPress={mockOnPress} 
        backgroundColor="#00ff00" 
        textColor="#ff0000" 
      />
    );
    
    const button = screen.getByText('Custom Colors').parentElement;
    const formattedText = screen.getByTestId('formatted-text');
    
    expect(button).toBeInTheDocument();
    expect(formattedText).toBeInTheDocument();
  });

  it('should render button with noBorder and noPadding options', () => {
    render(
      <Button 
        text="No Border" 
        onPress={mockOnPress} 
        noBorder={true} 
        noPadding={true} 
      />
    );
    
    const button = screen.getByText('No Border').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should render button with custom border radius', () => {
    render(<Button text="Rounded" onPress={mockOnPress} borderRadius={20} />);
    
    const button = screen.getByText('Rounded').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should have correct icon color based on theme', () => {
    render(<Button icon="test-icon" onPress={mockOnPress} />);
    
    const icon = screen.getByTestId('icon');
    expect(icon).toHaveAttribute('data-color', '#007acc');
  });

  it('should have correct disabled icon color', () => {
    render(<Button icon="test-icon" onPress={mockOnPress} isDisabled={true} />);
    
    const icon = screen.getByTestId('icon');
    expect(icon).toHaveAttribute('data-color', '#666666');
  });

  it('should have correct reversed icon color', () => {
    render(<Button icon="test-icon" onPress={mockOnPress} reverseColor={true} />);
    
    const icon = screen.getByTestId('icon');
    expect(icon).toHaveAttribute('data-color', '#2a2a2a');
  });

  it('should handle static button without pressable behavior', () => {
    render(<Button text="Static" onPress={mockOnPress} isStatic={true} />);
    
    const button = screen.getByText('Static').parentElement;
    expect(button).toBeInTheDocument();
  });

  it('should handle button with icon type correctly', () => {
    render(<Button icon="settings" onPress={mockOnPress} />);
    
    const icon = screen.getByTestId('icon');
    expect(icon).toHaveAttribute('data-type', 'settings');
  });
});