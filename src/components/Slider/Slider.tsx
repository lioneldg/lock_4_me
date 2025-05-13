import React from 'react';
import styles from './style.module.css';
import FormattedText from '../FormattedText';
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
        <FormattedText>{label} : </FormattedText>
        <span className={styles.value}>
          <FormattedText>
            {value} {unit}
          </FormattedText>
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
