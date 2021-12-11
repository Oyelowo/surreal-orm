import { AppProps } from "next/app";
import "../styles/globals.css";
import { GlobalStyles } from "twin.macro";
import { Provider } from "jotai";

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <div>

      <GlobalStyles />
      <Component {...pageProps} />

    </div>
  );
};

export default App;
