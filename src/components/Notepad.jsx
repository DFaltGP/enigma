import {
  Box,
  VStack,
  HStack,
  Text,
  IconButton,
} from '@chakra-ui/react';
import { FaEraser } from 'react-icons/fa';

const Notepad = ({ text, onClear }) => {
  const lines = text.split('\n');

  return (
    <Box position="relative" width="450px" flexShrink="0">
      <Box
        background="#654321"
        height="40px"
        position="relative"
        border="4px solid"
        borderColor="#4A3219"
      >
        <HStack
          position="absolute"
          left="0"
          top="0"
          bottom="0"
          spacing="1"
          paddingLeft="4"
          align="center"
        >
          {[...Array(15)].map((_, i) => (
            <Box
              key={i}
              width="8px"
              height="24px"
              background="#4A3219"
              borderRadius="sm"
            />
          ))}
        </HStack>
      </Box>

      <Box
        background="#F5E6D3"
        boxShadow="2xl"
        borderLeft="4px solid"
        borderRight="4px solid"
        borderBottom="4px solid"
        borderColor="#8B7765"
        minHeight="600px"
        position="relative"
        overflow="hidden"
        sx={{
          backgroundImage: `
            linear-gradient(to bottom, rgba(139, 119, 101, 0.05) 0%, transparent 100%),
            radial-gradient(ellipse at 0% 0%, rgba(139, 69, 19, 0.25) 0%, transparent 30%),
            radial-gradient(ellipse at 100% 0%, rgba(160, 82, 45, 0.2) 0%, transparent 25%),
            radial-gradient(ellipse at 0% 100%, rgba(139, 69, 19, 0.3) 0%, transparent 35%),
            radial-gradient(ellipse at 100% 100%, rgba(160, 82, 45, 0.28) 0%, transparent 30%)
          `,
        }}
      >
        <Box
          position="absolute"
          top="5%"
          left="3%"
          width="160px"
          height="128px"
          borderRadius="full"
          bg="#D2B48C"
          opacity="0.3"
          filter="blur(48px)"
          transform="rotate(-25deg) scale(1.4, 0.9)"
        />
        <Box
          position="absolute"
          top="10%"
          right="3%"
          width="144px"
          height="112px"
          borderRadius="full"
          background="#C4A574"
          opacity="0.25"
          filter="blur(48px)"
          transform="rotate(35deg) scale(1.2, 1.3)"
        />
        <Box
          position="absolute"
          bottom="15%"
          left="2%"
          width="192px"
          height="160px"
          borderRadius="full"
          background="#8B6914"
          opacity="0.2"
          filter="blur(48px)"
          transform="rotate(10deg) scale(1.5, 1)"
        />
        <Box
          position="absolute"
          bottom="10%"
          right="3%"
          width="176px"
          height="144px"
          borderRadius="full"
          background="#A0825A"
          opacity="0.22"
          filter="blur(48px)"
          transform="rotate(-30deg) scale(1.3, 1.2)"
        />

        <Box
          position="absolute"
          inset="0"
          pointerEvents="none"
          sx={{
            boxShadow:`
              inset 12px 12px 30px rgba(139, 69, 19, 0.25), 
              inset -12px -12px 30px rgba(139, 69, 19, 0.2), 
              inset 0 0 50px rgba(139, 119, 101, 0.15)
            `,
          }}
        />

        <VStack padding="8" spacing="4" align="stretch" position="relative" zIndex="10">
          <HStack justify="space-between" align="center" marginBottom="6">
            <Text
              color="#3D2817"
              letterSpacing="widest"
              fontSize="26px"
              fontWeight="normal"
            >
              NOTEPAD
            </Text>
            <IconButton
              icon={<FaEraser />}
              onClick={onClear}
              size="sm"
              variant="ghost"
              color="#654321"
              _hover={{ bg: '#E8DCC8' }}
              aria-label="Limpar"
            />
          </HStack>

          <VStack spacing="28px" align="stretch">
            {text ? (
              <Box
                height="400px"
                overflowY="auto"
                sx={{
                  '&::-webkit-scrollbar': {
                    width: '6px',
                  },
                  '&::-webkit-scrollbar-thumb': {
                    background: '#8B7765',
                    borderRadius: '3px',
                  },
                }}
                fontFamily="'Dancing Script', cursive"
                color="#3D2817"
              >
                {lines.map((line, i) => (
                  <Box
                    key={i}
                    borderBottom="1px solid"
                    borderColor="rgba(139, 119, 101, 0.3)"
                    paddingBottom="1"
                    minHeight="28px"
                    wordBreak="break-word"
                    fontSize="26px"
                  >
                    {line || '\u00A0'}
                  </Box>
                ))}
              </Box>
            ) : (
              <Box color="#8B7765" fontStyle="italic" fontFamily="Georgia, serif" fontSize="18px">
                <Text
                  borderBottom="1px solid"
                  borderColor="rgba(139, 119, 101, 0.3)"
                  paddingBottom="1"
                >
                  Digite usando o teclado da máquina Enigma...
                </Text>
                {[...Array(15)].map((_, i) => (
                  <Box
                    key={i}
                    borderBottom="1px solid"
                    borderColor="rgba(139, 119, 101, 0.3)"
                    height="28px"
                  />
                ))}
              </Box>
            )}
          </VStack>
        </VStack>
      </Box>
    </Box>
  );
};

export default Notepad;