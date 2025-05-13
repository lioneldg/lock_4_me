import React from 'react';
import styles from './style.module.css';
import { useTheme } from '../../hooks/ThemeContext';
import FormattedText from '../FormattedText';

export interface DropdownOption {
  value: string;
  label: string;
}

interface DropdownProps {
  options: DropdownOption[];
  value: string;
  onChange: (value: string) => void;
  label?: string;
}

const Dropdown: React.FC<DropdownProps> = ({ options, value, onChange, label }) => {
  const { colors } = useTheme();

  return (
    <div>
      {label && (
        <label className={styles.label}>
          <FormattedText>{label}</FormattedText>
        </label>
      )}
      <select
        className={styles.select}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        style={{
          background: colors.backgroundColor,
          color: colors.backgroundTextColor,
          borderColor: colors.borderColor
        }}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
};

export default Dropdown;
