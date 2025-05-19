import React from 'react';
import { useTheme } from '../../hooks/ThemeContext';
import { useTranslation } from 'react-i18next';
import styles from './style.module.css';
import FormattedText from '../FormattedText';
const LoadingSpinner: React.FC = () => {
  const { colors } = useTheme();
  const { t } = useTranslation();
  const circleColor = colors.accentColor;
  return (
    <div
      className={styles.loading_spinner}
      style={{
        color: circleColor,
        backgroundColor: colors.backgroundColor
      }}
    >
      <div style={{ marginBottom: 24 }}>
        <svg
          width="48"
          height="48"
          viewBox="0 0 48 48"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <circle
            cx="24"
            cy="24"
            r="20"
            stroke={circleColor}
            strokeWidth="4"
            strokeDasharray="100"
            strokeDashoffset="60"
            strokeLinecap="round"
          >
            <animateTransform
              attributeName="transform"
              type="rotate"
              from="0 24 24"
              to="360 24 24"
              dur="1s"
              repeatCount="indefinite"
            />
          </circle>
        </svg>
      </div>
      <FormattedText style={{ fontSize: 24 }}>{t('loading')}</FormattedText>
    </div>
  );
};

export default LoadingSpinner;
