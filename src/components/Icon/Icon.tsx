import React from 'react';
import Ic from '@mdi/react';
import { useTheme } from '../../hooks/ThemeContext';
import {
  mdiArrowLeftRightBold,
  mdiArrowLeftBold,
  mdiArrowRightBold,
  mdiPencilOutline,
  mdiCogs,
  mdiClose,
  mdiCloudUploadOutline,
  mdiCloudDownloadOutline,
  mdiMagnify,
  mdiSwapHorizontal,
  mdiRefresh,
  mdiHistory,
  mdiSendOutline,
  mdiDownload,
  mdiUpload,
  mdiDelete,
  mdiFolderUploadOutline,
  mdiFolderDownloadOutline,
  mdiDotsHorizontal,
  mdiWeatherNight,
  mdiWhiteBalanceSunny,
  mdiFolderOutline,
  mdiFolderSync,
  mdiFilterVariant,
  mdiCheck,
  mdiLogout,
  mdiContentSaveEditOutline,
  mdiLockOutline
} from '@mdi/js';

export const ICON_TYPE = {
  ARROW_LEFT_BOLD: mdiArrowLeftBold,
  ARROW_RIGHT_BOLD: mdiArrowRightBold,
  ARROW_LEFT_RIGHT_BOLD: mdiArrowLeftRightBold,
  DARK_MODE: mdiWeatherNight,
  CANCEL: mdiClose,
  CHECK: mdiCheck,
  DELETE: mdiDelete,
  DIFF: mdiSwapHorizontal,
  DOTS_HORIZONTAL: mdiDotsHorizontal,
  DOWNLOAD: mdiDownload,
  DOWNLOAD_CLOUD: mdiCloudDownloadOutline,
  EDIT: mdiPencilOutline,
  FILTER: mdiFilterVariant,
  FOLDER: mdiFolderOutline,
  FOLDER_DOWNLOAD: mdiFolderDownloadOutline,
  FOLDER_UPLOAD: mdiFolderUploadOutline,
  FOLDER_REFRESH: mdiFolderSync,
  HISTORY: mdiHistory,
  LIGHT_MODE: mdiWhiteBalanceSunny,
  LOCK: mdiLockOutline,
  LOGOUT: mdiLogout,
  REFRESH: mdiRefresh,
  SAVE: mdiContentSaveEditOutline,
  SEARCH: mdiMagnify,
  SEND: mdiSendOutline,
  SETTINGS: mdiCogs,
  UPLOAD_CLOUD: mdiCloudUploadOutline,
  UPLOAD: mdiUpload
};

export type IconType = (typeof ICON_TYPE)[keyof typeof ICON_TYPE];

interface propsInterface {
  type: IconType;
  size?: number;
  color?: string;
  title?: string;
  flipHorizontal?: boolean;
  flipVertical?: boolean;
  rotate?: number;
  spin?: boolean | number;
  style?: string;
}

const Icon: React.FC<propsInterface> = (props: propsInterface) => {
  const { colors } = useTheme();

  const {
    type,
    size = 1,
    color = colors.accentColor,
    title,
    flipHorizontal,
    flipVertical,
    rotate,
    spin,
    style
  } = props;
  return (
    <Ic
      path={type}
      title={title}
      size={size}
      horizontal={flipHorizontal}
      vertical={flipVertical}
      rotate={rotate}
      color={color}
      spin={spin}
      className={style}
    />
  );
};

export default Icon;
