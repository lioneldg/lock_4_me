import React from 'react';
import { render } from '@testing-library/react';
import { screen } from '@testing-library/dom';
import { describe, it, expect, jest } from '@jest/globals';
import '@testing-library/jest-dom';
import LoadingSpinner from '../src/components/LoadingSpinner';

// Mock the hooks and components used in LoadingSpinner
jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => ({
    colors: {
      backgroundColor: '#1a1a1a',
      accentColor: '#007acc'
    }
  })
}));

jest.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => (key === 'loading' ? 'Loading...' : key)
  })
}));

jest.mock('../src/components/FormattedText', () => ({
  __esModule: true,
  default: ({ children, style }: { children: React.ReactNode; style?: React.CSSProperties }) => (
    <span data-testid="formatted-text" style={style}>
      {children}
    </span>
  )
}));

describe('LoadingSpinner Component', () => {
  it('should render the loading spinner with correct structure', () => {
    render(<LoadingSpinner />);

    // Check if the loading text is in the document
    expect(screen.getByText('Loading...')).toBeInTheDocument();
    expect(screen.getByTestId('formatted-text')).toBeInTheDocument();

    // Check if the SVG element is in the document with correct attributes
    const svgElement = document.querySelector('svg');
    expect(svgElement).toBeInTheDocument();
    expect(svgElement).toHaveAttribute('width', '48');
    expect(svgElement).toHaveAttribute('height', '48');
    expect(svgElement).toHaveAttribute('viewBox', '0 0 48 48');
  });

  it('should render circle with correct attributes and animation', () => {
    render(<LoadingSpinner />);

    // Check if the circle element is in the document with correct attributes
    const circleElement = document.querySelector('circle');
    expect(circleElement).toBeInTheDocument();
    expect(circleElement).toHaveAttribute('cx', '24');
    expect(circleElement).toHaveAttribute('cy', '24');
    expect(circleElement).toHaveAttribute('r', '20');
    expect(circleElement).toHaveAttribute('stroke', '#007acc');
    expect(circleElement).toHaveAttribute('stroke-width', '4');
    expect(circleElement).toHaveAttribute('stroke-dasharray', '100');
    expect(circleElement).toHaveAttribute('stroke-dashoffset', '60');
    expect(circleElement).toHaveAttribute('stroke-linecap', 'round');

    // Check if the animation element exists
    const animationElement = document.querySelector('animateTransform');
    expect(animationElement).toBeInTheDocument();
    expect(animationElement).toHaveAttribute('attributeName', 'transform');
    expect(animationElement).toHaveAttribute('type', 'rotate');
    expect(animationElement).toHaveAttribute('dur', '1s');
    expect(animationElement).toHaveAttribute('repeatCount', 'indefinite');
  });

  it('should apply theme colors correctly', () => {
    render(<LoadingSpinner />);

    // Check that the circle uses the correct accent color from theme
    const circleElement = document.querySelector('circle');
    expect(circleElement).toHaveAttribute('stroke', '#007acc');

    // Check if FormattedText has correct styling
    const formattedText = screen.getByTestId('formatted-text');
    expect(formattedText).toHaveStyle('font-size: 24px');
  });

  it('should use translation hook correctly', () => {
    render(<LoadingSpinner />);

    // Verify that the translation function was called with the correct key
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
});
