import { MantineProvider } from '@mantine/core';
import type { AppProps } from 'next/app';
import Head from 'next/head';
import CpHeader from '../components/CpHeader';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <MantineProvider
      theme={{ colorScheme: 'dark', fontFamily: 'Montserrat' }}
    >
      <CpHeader />
      <Component {...pageProps} />
    </MantineProvider>
  );
}

export default MyApp;
