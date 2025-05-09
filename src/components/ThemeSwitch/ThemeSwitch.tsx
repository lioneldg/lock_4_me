import React from 'react';
import style from './ThemeSwitch.module.css';
import { useTheme } from '../../hooks/ThemeContext';
import Icon, { ICON_TYPE } from '../Icon/Icon';
import { useSettingsStore } from '../../store/settingsStore';

const LIGHT = 'light';
const DARK = 'dark';

const ThemeSwitch: React.FC = () => {
  const { theme, setTheme, colors } = useTheme();
  const { setSettings } = useSettingsStore();

  function switchTheme(): void {
    const newTheme = theme === LIGHT ? DARK : LIGHT;
    setTheme(newTheme);
    setSettings({ theme: newTheme });
  }

  const isLight = theme === LIGHT;
  const iconType = theme === LIGHT ? ICON_TYPE.LIGHT_MODE : ICON_TYPE.DARK_MODE;

  const boxStyle = {
    border: `1px solid ${colors.accentColor}`
  };

  return (
    <div className={style.box} style={boxStyle} onClick={switchTheme}>
      <div className={`${style.switch} ${isLight ? style.active : ''}`}>
        <span className={style.slider}>
          <Icon type={iconType} size={1} color={colors.accentColor} />
        </span>
      </div>
    </div>
  );
};

export default ThemeSwitch;
