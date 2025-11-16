import {
  Box,
  VStack,
  HStack,
  Text,
  Button,
  IconButton,
  Flex,
} from '@chakra-ui/react';
import { FaTrash } from 'react-icons/fa';
import { motion } from 'framer-motion';

const MotionButton = motion(Button);

const EnigmaMachine = ({ encryptedText, onKeyPress, onClear }) => {
  const keyboardRows = [
    ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
    ['Z', 'X', 'C', 'V', 'B', 'N', 'M']
  ];

  return (
    <Box
      width="650px"
      flexShrink="0"
      bgGradient="linear(to-b, #654321, #5C3D2E, #4A3219)"
      borderRadius="lg"
      boxShadow="0 20px 50px rgba(0,0,0,0.5), inset 0 2px 4px rgba(139, 105, 20, 0.3)"
      padding="8"
      border="8px solid"
      borderColor="#8B6914"
      position="relative"
    >
      {[
        { top: '16px', left: '16px' },
        { top: '16px', right: '16px' },
        { top: '33.3333%', left: '16px' },
        { top: '33.3333%', right: '16px' },
        { top: '66.6667%', left: '16px' },
        { top: '66.6667%', right: '16px' },
        { bottom: '16px', left: '16px' },
        { bottom: '16px', right: '16px' },
      ].map((pos, i) => (
        <Box
          key={i}
          position="absolute"
          {...pos}
          width="12px"
          height="12px"
          borderRadius="full"
          bg="#3D2817"
          border="1px solid"
          borderColor="#1a1a1a"
          zIndex={20}
        >
          <Box
            position="absolute"
            inset="0"
            display="flex"
            alignItems="center"
            justifyContent="center"
          >
            <Box width="8px" height="2px" bg="#1a1a1a" />
          </Box>
        </Box>
      ))}
      
      <Box
        position="absolute"
        inset="0"
        opacity="0.1"
        borderRadius="lg"
        pointerEvents="none"
        sx={{
          backgroundImage: `repeating-linear-gradient(
            90deg, 
            rgba(0,0,0,0.1) 0px, 
            transparent 1px, 
            transparent 3px, 
            rgba(0,0,0,0.1) 4px
          )`,
        }}
      />
      
      <Box marginBottom="8" position="relative" zIndex={10}>
        <Box
          background="#F5E6D3"
          borderRadius="md"
          p="6"
          boxShadow="inner"
          border="4px solid"
          borderColor="#3D2817"
          minHeight="200px"
        >
          <HStack justify="space-between" align="center" marginBottom="4">
            <Text
              color="#3D2817"
              letterSpacing="widest"
              fontSize="22px"
              fontWeight="normal"
            >
              TEXTO CRIPTOGRAFADO
            </Text>
            <IconButton
              icon={<FaTrash />}
              onClick={onClear}
              size="sm"
              variant="ghost"
              color="#8B4513"
              _hover={{ bg: '#E8DCC8' }}
              aria-label="Limpar tudo"
            />
          </HStack>
          
          <Box
            background="#1a1a1a"
            borderRadius="sm"
            p="4"
            height="120px"
            overflowY="auto"
            sx={{
              '&::-webkit-scrollbar': { width: '6px' },
              '&::-webkit-scrollbar-thumb': { background: '#3D2817', borderRadius: '3px' },
            }}
            fontFamily="'Courier New', monospace"
            fontSize="20px"
            border="2px solid"
            borderColor="#3D2817"
          >
            <Text color="#D4AF37" wordBreak="break-word" letterSpacing="wide">
              {encryptedText || (
                <Text as="span" color="#8B7355" fontStyle="italic">
                  Aguardando entrada...
                </Text>
              )}
            </Text>
          </Box>
        </Box>
      </Box>

      <VStack spacing="3" align="stretch" position="relative" zIndex="10">
        {keyboardRows.map((row, rowIndex) => (
          <Flex
            key={rowIndex}
            justify="center"
            gap={2}
            paddingLeft={rowIndex === 1 ? '24px' : rowIndex === 2 ? '48px' : '0'}
          >
            {row.map((key) => (
              <MotionButton
                key={key}
                onClick={() => onKeyPress(key)}
                width="48px"
                height="48px"
                bgGradient="linear(to-b, #D4AF37, #B8941E)"
                borderRadius="full"
                boxShadow="0 4px 8px rgba(0,0,0,0.4), inset 0 1px 2px rgba(255,255,255,0.2)"
                border="4px solid"
                borderColor="#8B6914"
                color="#3D2817"
                fontSize="lg"
                fontWeight="normal"
                userSelect="none"
                _hover={{
                  bgGradient: 'linear(to-b, #E5C158, #D4AF37)',
                }}
                _active={{
                  bgGradient: 'linear(to-b, #B8941E, #9A7A1A)',
                }}
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                transition={{ duration: 0.1 }}
              >
                {key}
              </MotionButton>
            ))}
          </Flex>
        ))}
        
        <Flex justify="center" paddingTop={2}>
          <MotionButton
            onClick={() => onKeyPress(' ')}
            width="256px"
            height="48px"
            bgGradient="linear(to-b, #D4AF37, #B8941E)"
            borderRadius="full"
            boxShadow="0 4px 8px rgba(0,0,0,0.4), inset 0 1px 2px rgba(255,255,255,0.2)"
            border="4px solid"
            borderColor="#8B6914"
            color="#3D2817"
            fontSize="md"
            fontWeight="normal"
            userSelect="none"
            _hover={{
              bgGradient: 'linear(to-b, #E5C158, #D4AF37)',
            }}
            _active={{
              bgGradient: 'linear(to-b, #B8941E, #9A7A1A)',
            }}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            transition={{ duration: 0.1 }}
          >
            ESPAÇO
          </MotionButton>
        </Flex>
      </VStack>

      <Box marginTop="8" position="relative" zIndex="10">
        <Flex justify="space-around" paddingX="12">
          {[...Array(8)].map((_, i) => (
            <Box
              key={i}
              width="8px"
              height="8px"
              borderRadius="full"
              background="#3D2817"
              border="1px solid"
              borderColor="#1a1a1a"
              boxShadow="inner"
            />
          ))}
        </Flex>
      </Box>
    </Box>
  );
};

export default EnigmaMachine;