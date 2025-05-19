import React, { PropsWithChildren } from 'react';
import styleLocal from './style.module.css';
import { useTheme } from '../../hooks/ThemeContext';

interface FormattedTextProps {
  style?: React.CSSProperties;
}

const FormattedText: React.FC<PropsWithChildren<FormattedTextProps>> = ({ children, style }) => {
  const { colors } = useTheme();
  return (
    <div className={styleLocal.container} style={{ color: colors.accentColor, ...style }}>
      {children}
    </div>
  );
};

export default FormattedText;
