import { useEffect, useState } from "react";
import { lightColors, darkColors } from "../libs/colors";
import { Colors, Theme } from "../types";

export function useTheme(): {
  theme: Theme;
  colors: Colors;
  setTheme: (theme: Theme) => void;
} {
  const [theme, setTheme] = useState<Theme>(() => {
    if (
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
      return "dark";
    }
    return "light";
  });

  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = () => {
      setTheme(mediaQuery.matches ? "dark" : "light");
    };
    mediaQuery.addEventListener("change", handleChange);
    return () => mediaQuery.removeEventListener("change", handleChange);
  }, []);

  const colors = theme === "dark" ? darkColors : lightColors;

  return { theme, colors, setTheme };
}
