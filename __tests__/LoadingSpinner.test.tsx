import React from 'react';
import { render } from '@testing-library/react';
import { screen } from '@testing-library/dom';
import { describe, it, expect, jest } from '@jest/globals';
import '@testing-library/jest-dom';
import LoadingSpinner from '../src/components/LoadingSpinner';

// Mock the hooks used in the component
jest.mock('../src/hooks/ThemeContext', () => ({
  useTheme: () => ({
    colors: {
      background: '#ffffff',
      accentColor: '#000000'
    }
  })
}));

jest.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => (key === 'loading' ? 'Loading...' : key)
  })
}));

describe('LoadingSpinner Component', () => {
  it('should render the loading spinner with text', () => {
    render(<LoadingSpinner />);

    // Check if the loading text is in the document
    expect(screen.getByText('Loading...')).toBeInTheDocument();

    // Check if the SVG element is in the document
    const svgElement = document.querySelector('svg');
    expect(svgElement).toBeInTheDocument();

    // Check if the circle element is in the document with correct attributes
    const circleElement = document.querySelector('circle');
    expect(circleElement).toBeInTheDocument();
    expect(circleElement).toHaveAttribute('stroke', '#000000');
  });
});
