import React from 'react';
import styleLocal from './style.module.css';
import { useTheme } from '../../hooks/ThemeContext';

interface FormattedTextProps {
  children: React.ReactNode;
  style?: React.CSSProperties;
}

const FormattedText: React.FC<FormattedTextProps> = ({ children, style }) => {
  const { colors } = useTheme();
  return (
    <div className={styleLocal.container} style={{ color: colors.backgroundTextColor, ...style }}>
      {children}
    </div>
  );
};

export default FormattedText;
