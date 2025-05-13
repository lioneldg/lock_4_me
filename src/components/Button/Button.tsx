import React from 'react';
import styles from './style.module.css';
import Icon from '../Icon';
import { IconType } from '../Icon/Icon';
import FormattedText from '../FormattedText';
import { useTheme } from '../../hooks/ThemeContext';

interface ButtonProps {
  icon?: IconType;
  text?: string;
  onPress: () => void;
  width?: number;
  height?: number;
  textColor?: string;
  backgroundColor?: string;
  borderRadius?: number;
  noBorder?: boolean;
  noPadding?: boolean;
  isStatic?: boolean;
  isDisabled?: boolean;
  reverseColor?: boolean;
}
enum BUTTON_COLOR_STATUS {
  NORMAL,
  REVERSE,
  DISABLED
}

const Button: React.FC<ButtonProps> = (props) => {
  const {
    icon,
    text,
    width,
    height,
    textColor,
    backgroundColor,
    onPress,
    borderRadius,
    noBorder,
    noPadding,
    isStatic,
    isDisabled,
    reverseColor
  } = props;
  const { colors } = useTheme();

  const buttonColorStatus = isDisabled
    ? BUTTON_COLOR_STATUS.DISABLED
    : reverseColor
      ? BUTTON_COLOR_STATUS.REVERSE
      : BUTTON_COLOR_STATUS.NORMAL;

  let _backgroundColor = text ? backgroundColor : 'transparent';
  let _textColor = textColor;

  switch (buttonColorStatus) {
    case BUTTON_COLOR_STATUS.DISABLED:
      _backgroundColor ??= colors.secondaryBackgroundColor;
      _textColor ??= colors.disabledColor;
      break;
    case BUTTON_COLOR_STATUS.REVERSE:
      _backgroundColor ??= colors.accentColor;
      _textColor ??= colors.secondaryBackgroundColor;
      break;
    default:
      _backgroundColor ??= colors.secondaryBackgroundColor;
      _textColor ??= colors.accentColor;
  }

  const propsStyle = {
    width: width ? `${width}rem` : text ? '7rem' : '0.1rem',
    height: height ? `${height}rem` : text ? '2rem' : '1.5rem',
    backgroundColor: _backgroundColor,
    border: `${noBorder || !text ? 0 : 1}px solid ${isDisabled ? colors.text : colors.accentColor}`,
    padding: noPadding ? '0rem' : '',
    cursor: isDisabled ? 'default' : 'pointer',
    borderRadius: `${borderRadius}px`
  };

  const handleOnPress = (): void => {
    if (!isDisabled) {
      onPress();
    }
  };

  return (
    <div
      className={`button ${!isStatic && !isDisabled && 'pressable'} ${styles.container}`}
      style={propsStyle}
      onClick={handleOnPress}
    >
      {icon && (
        <div className={`icon ${styles.icon}`}>
          <Icon type={icon} size={1} color={_textColor} />
        </div>
      )}
      {text && (
        <div className={styles.text}>
          <FormattedText style={{ color: _textColor }}>{text}</FormattedText>
        </div>
      )}
    </div>
  );
};

export default Button;
