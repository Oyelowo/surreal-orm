import createCache from "@emotion/cache";
import { CacheProvider } from "@emotion/react";
import { themes } from "@storybook/theming";
import { GlobalStyles, theme } from "twin.macro";
import "../styles/globals.css";

const cache = createCache({ prepend: true, key: "twin" });

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
  // layout: "centered",
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
    expanded: true,
  },
  // backgrounds: {
  //   default: "electric-ribbon",
  //   values: [
  //     {
  //       name: "electric-ribbon",
  //       value: `linear-gradient(180deg, ${theme`colors.electric`}, ${theme`colors.ribbon`})`,
  //     },
  //   ],
  // },
  // Override the default dark theme
  dark: { ...themes.dark, appBg: "black" },
  // Override the default light theme
  light: { ...themes.normal, appBg: "red" },
};

export const decorators = [
  Story => (
    <CacheProvider value={cache}>
      <GlobalStyles />
      <Story />
    </CacheProvider>
  ),
];
