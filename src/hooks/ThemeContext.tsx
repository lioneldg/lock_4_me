import React, { createContext, useContext, useState } from 'react';
import { lightColors, darkColors } from '../libs/colors';
import { Colors, Theme } from '../types';

const ThemeContext = createContext<{
  theme: Theme;
  colors: Colors;
  setTheme: (theme: Theme) => void;
}>({
  theme: 'dark',
  colors: darkColors,
  setTheme: () => {}
});

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setTheme] = useState<Theme>('dark');
  const colors = theme === 'dark' ? darkColors : lightColors;

  return (
    <ThemeContext.Provider value={{ theme, colors, setTheme }}>{children}</ThemeContext.Provider>
  );
};

export const useTheme = () => useContext(ThemeContext);
