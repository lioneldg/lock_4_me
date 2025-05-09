import React from 'react';
import styles from './style.module.css';
import { useTheme } from '../../hooks/ThemeContext';

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
    <div className={styles.dropdownContainer}>
      {label && (
        <label className={styles.label} style={{ color: colors.text }}>
          {label}
        </label>
      )}
      <select
        className={styles.select}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        style={{
          background: colors.surface,
          color: colors.onSurface,
          borderColor: colors.primary
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
