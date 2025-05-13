import React from 'react';
import Button from './Button';
import { ICON_TYPE } from '../Icon/Icon';
import { useNavigate } from 'react-router';

interface BackToMainProps {
  isDisabled?: boolean;
}

const BackToMain: React.FC<BackToMainProps> = ({ isDisabled }) => {
  const navigate = useNavigate();
  const goBack = () => navigate('/');
  return (
    <Button
      icon={ICON_TYPE.ARROW_LEFT_BOLD}
      text=""
      onPress={goBack}
      height={2}
      width={2}
      noPadding
      isDisabled={isDisabled}
    />
  );
};

export default BackToMain;
