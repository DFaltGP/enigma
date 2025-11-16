import { extendTheme } from '@chakra-ui/react';

const theme = extendTheme({
  config: {
    initialColorMode: 'light',
    useSystemColorMode: false,
  },
  colors: {
    vintage: {
      beige: '#E8DCC8',
      cream: '#F5E6D3',
      darkBrown: '#3D2817',
      brown: '#5C4033',
      gold: '#D4AF37',
      paper: '#F5E6D3',
      binding: '#654321',
      bindingDark: '#4A3219',
      border: '#8B7765',
      machine: {
        dark: '#654321',
        medium: '#5C3D2E',
        light: '#4A3219',
        border: '#8B6914',
      },
    },
  },
  styles: {
    global: {
      body: {
        bg: '#E8DCC8',
        color: '#3D2817',
        fontFamily: "'EB Garamond', serif",
      },
    },
  },
});

export default theme;