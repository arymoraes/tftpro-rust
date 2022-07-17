import { MantineProvider } from '@mantine/core';
import type { AppProps } from 'next/app';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <MantineProvider theme={{ colorScheme: 'dark' }}>
      <Component {...pageProps} />
    </MantineProvider>
  );
}

export default MyApp;
