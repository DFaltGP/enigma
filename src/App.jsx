import { useEffect, useState } from 'react';
import { Box, Container, Flex, Heading, VStack } from '@chakra-ui/react';
import { invoke } from '@tauri-apps/api/core';
import Notepad from './components/Notepad';
import EnigmaMachine from './components/EnigmaMachine';
import paperTexture from './assets/old-paper.jpg';

export default function App() {
  const [originalText, setOriginalText] = useState('');
  const [encryptedText, setEncryptedText] = useState('');

  const getDefaultConfig = () => ({
    rotors: [
      { name: 'I', position: 'A', ring: 'A' },    
      { name: 'II', position: 'A', ring: 'A' },   
      { name: 'III', position: 'A', ring: 'A' },  
    ],
    reflector: 'B',
    plugboard_pairs: '',
  });

const handleKeyPress = async (key) => {
  if (key === ' ') {
    setOriginalText(prev => prev + ' ');
    setEncryptedText(prev => prev + ' ');
    return;
  }

  if (!/^[A-Za-z]$/.test(key)) return;

  setOriginalText(prev => {
    const updated = prev + key.toLowerCase();

    processEncryption(updated);

    return updated;
  });
};

const processEncryption = async (text) => {
  try {
    const encrypted = await invoke('enigma_process_string', {
      config: getDefaultConfig(),
      text: text.toUpperCase(),
    });

    setEncryptedText(encrypted);
  } catch {
    setEncryptedText(prev => prev + '?');
  }
};

  const handleClear = () => {
    setOriginalText('');
    setEncryptedText('');
  };

  useEffect(() => {
    const handlePhysicalKeyPress = (event) => {
      handleKeyPress(event.key);
    };

    window.addEventListener('keydown', handlePhysicalKeyPress);

    return () => {
      window.removeEventListener('keydown', handlePhysicalKeyPress);
    };
  }, []);

  return (
    <Box
      minHeight="100vh"
      background="#E8DCC8"
      padding="8"
      position="relative"
      overflow="hidden"
      sx={{
        backgroundImage: `url(${paperTexture})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
      }}
    >
      <Box
        position="absolute"
        inset="0"
        background="rgba(232, 220, 200, 0.3)" 
        pointerEvents="none"
      />
      
      <Container maxWidth="7xl" marginX="auto" position="relative" zIndex="10">
        <Heading
          as="h1"
          textAlign="center"
          color="#3D2817"
          marginBottom="2"
          letterSpacing="wider"
          fontFamily="'EB Garamond', serif"
          fontSize="50px"
          fontWeight="extrabold"
        >
          ENIGMA 
        </Heading>
        <Heading
          as="h1"
          textAlign="center"
          color="#3D2817"
          marginBottom="2"
          letterSpacing="wider"
          fontFamily="'EB Garamond', serif"
          fontSize="25px"
          fontWeight="extrabold"
        >
          CRIPTOGRAFADOR DE MENSAGENS DE TEXTO 
        </Heading>
        
        <Flex
          direction={{ base: 'column', lg: 'row' }}
          gap="8"
          align="stretch"
          justify="center"
          marginTop="6"
        >
          <Notepad 
            text={originalText} 
            onClear={handleClear}
          />
          <EnigmaMachine 
            encryptedText={encryptedText}
            onKeyPress={handleKeyPress}
            onClear={handleClear}
          />
        </Flex>
      </Container>
    </Box>
  );
};