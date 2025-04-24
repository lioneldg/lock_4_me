import React from "react";
import { useTheme } from "../hooks/ThemeContext";
import { useTranslation } from "react-i18next";

const LoadingSpinner: React.FC = () => {
  const { colors } = useTheme();
  const { t } = useTranslation();
  const background = colors.background;
  const textColor = colors.accentColor;
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        height: "100vh",
        background,
        color: textColor,
        fontSize: 32,
        flexDirection: "column",
        transition: "background 0.2s, color 0.2s",
      }}
    >
      <div className="loader" style={{ marginBottom: 24 }}>
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
            stroke={textColor}
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
      {t("loading")}
    </div>
  );
};

export default LoadingSpinner;
