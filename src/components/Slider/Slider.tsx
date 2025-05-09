import React from 'react';
import styles from './style.module.css';
interface SliderProps {
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  onChange: (value: number) => void;
  id?: string;
  style?: React.CSSProperties;
  unit?: string;
}

const Slider: React.FC<SliderProps> = ({
  label,
  value,
  min = 1,
  max = 50,
  step = 1,
  onChange,
  id,
  style,
  unit
}) => {
  return (
    <div className={styles.sliderContainer} style={style}>
      <label htmlFor={id} className={styles.label}>
        {label}:{' '}
        <span className={styles.value}>
          {value} {unit}
        </span>
      </label>
      <input
        id={id}
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className={styles.slider}
      />
    </div>
  );
};

export default Slider;
