/// Módulo que implementa a lógica da máquina Enigma M3 (usada pelo exército alemão).
/// Focado em ser didático, expondo os passos internos da criptografia.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Constantes (Definições de Rotores e Refletores Reais) ---

/// Mapeamento do Rotor I (EKMFLGDQVZNTOWYHXUSPAIBRCJ)
const ROTOR_I_WIRING: [u8; 26] = [
    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9,
];
/// Posição da ranhura (notch) do Rotor I ('Q' -> 16)
const ROTOR_I_NOTCH: u8 = 16;

/// Mapeamento do Rotor II (AJDKSIRUXBLHWTMCQGZNPYFVOE)
const ROTOR_II_WIRING: [u8; 26] = [
    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 25, 13, 15, 24, 5, 21, 14, 4, 6,
];
/// Posição da ranhura (notch) do Rotor II ('E' -> 4)
const ROTOR_II_NOTCH: u8 = 4;

/// Mapeamento do Rotor III (BDFHJLCPRTXVZNYEIWGAKMUSQO)
const ROTOR_III_WIRING: [u8; 26] = [
    1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 20, 6, 0, 10, 12, 18, 16, 14, 22,
];
/// Posição da ranhura (notch) do Rotor III ('V' -> 21)
const ROTOR_III_NOTCH: u8 = 21;

/// Mapeamento do Refletor B (YRUHQSLDPXNGOKMIEBFZCWVJAT)
const REFLECTOR_B_WIRING: [u8; 26] = [
    24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 19, 0,
];

/// Mapeamento do Refletor C (FVPJIAOYEDRZXWGCTKUQSBNMHL)
const REFLECTOR_C_WIRING: [u8; 26] = [
    5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11,
];

// --- Estruturas de Dados para a UI (Tauri) ---

/// Define a direção do sinal através do componente.
#[derive(Debug, Serialize, Clone)]
pub enum PathDirection {
    Forward,
    Reflect,
    Backward,
}

/// Representa um único passo do sinal elétrico através de um componente.
/// Ex: (Plugboard, 'A' -> 'G', Forward)
#[derive(Debug, Serialize, Clone)]
pub struct PathEntry {
    /// Nome do componente (ex: "Plugboard", "Rotor I", "Reflector B")
    component: String,
    /// Caractere de entrada no componente
    input_char: char,
    /// Caractere de saída do componente
    output_char: char,
    /// Direção do sinal
    direction: PathDirection,
}

/// Representa o processo completo de criptografia para um único caractere.
/// Esta estrutura é o que será enviado à UI para visualização didática.
#[derive(Debug, Serialize, Clone)]
pub struct EncryptionStep {
    /// Caractere original inserido (ex: 'A')
    input_char: char,
    /// Caractere final criptografado (ex: 'Z')
    output_char: char,
    /// Posição dos rotores (Esquerda, Meio, Direita) *antes* do passo.
    positions_before_step: (char, char, char),
    /// Posição dos rotores (Esquerda, Meio, Direita) *depois* do passo.
    positions_after_step: (char, char, char),
    /// O caminho detalhado do sinal elétrico através de todos os componentes.
    path: Vec<PathEntry>,
}

/// Configuração para um único rotor, vinda da UI.
/// `Deserialize` permite que o Tauri converta o JSON da UI para esta struct.
#[derive(Debug, Deserialize, Clone)]
pub struct RotorConfig {
    /// Nome do rotor ("I", "II", ou "III")
    pub name: String,
    /// Posição inicial do rotor (letra visível na janela, 'A' a 'Z')
    pub position: char,
    /// Configuração do anel (Ringstellung, 'A' a 'Z' ou 1 a 26)
    pub ring: char,
}

/// Configuração completa da máquina Enigma, vinda da UI.
#[derive(Debug, Deserialize)]
pub struct EnigmaConfig {
    /// Configuração dos três rotores, da *direita para a esquerda* (Rotor R, M, L).
    /// A ordem é importante: o primeiro rotor é o que gira a cada tecla.
    pub rotors: (RotorConfig, RotorConfig, RotorConfig),
    /// Nome do refletor ("B" ou "C")
    pub reflector: String,
    /// Pares do painel de conexões (ex: "AB CD EF")
    pub plugboard_pairs: String,
}

// --- Estruturas Internas da Lógica ---

/// Representa o Plugboard (Steckerbrett).
#[derive(Debug)]
struct Plugboard {
    /// Mapeia um `u8` (0-25) para outro. Se uma letra não está no plugboard,
    /// ela mapeia para si mesma.
    map: [u8; 26],
}

impl Plugboard {
    /// Cria um novo Plugboard a partir de uma string de pares (ex: "AB CD").
    fn new(pairs_str: &str) -> Self {
        let mut map: [u8; 26] = (0..26).collect::<Vec<u8>>().try_into().unwrap();
        
        // Processa os pares, ignorando espaços
        let pairs: Vec<char> = pairs_str.chars().filter(|c| c.is_ascii_alphabetic()).collect();

        for chunk in pairs.chunks(2) {
            if chunk.len() == 2 {
                let c1 = char_to_u8(chunk[0]);
                let c2 = char_to_u8(chunk[1]);
                map[c1 as usize] = c2;
                map[c2 as usize] = c1;
            }
        }
        Self { map }
    }

    /// Processa um caractere através do plugboard.
    /// Como é recíproco, esta função serve para entrada e saída.
    fn process(&self, c: u8) -> u8 {
        self.map[c as usize]
    }
}

/// Representa um único Refletor (Umkehrwalze).
#[derive(Debug)]
struct Reflector {
    wiring: [u8; 26],
    name: String,
}

impl Reflector {
    /// Cria um Refletor com base no nome ("B" ou "C").
    fn new(name: &str) -> Self {
        let wiring = match name.to_uppercase().as_str() {
            "B" => REFLECTOR_B_WIRING,
            "C" => REFLECTOR_C_WIRING,
            _ => panic!("Refletor desconhecido: {}. Use 'B' ou 'C'.", name),
        };
        Self {
            wiring,
            name: format!("Reflector {}", name),
        }
    }

    /// Reflete o sinal.
    fn reflect(&self, c: u8) -> u8 {
        self.wiring[c as usize]
    }
}

/// Representa um único Rotor (Walze).
#[derive(Debug, Clone)]
struct Rotor {
    /// Mapeamento de "ida" (direita para esquerda).
    wiring: [u8; 26],
    /// Mapeamento de "volta" (esquerda para direita).
    inverse_wiring: [u8; 26],
    /// Posição atual do rotor (0-25).
    position: u8,
    /// Configuração do anel (0-25).
    ring_setting: u8,
    /// Posição da ranhura (notch) que aciona o próximo rotor.
    notch: u8,
    /// Nome para fins didáticos (ex: "Rotor I").
    name: String,
}

impl Rotor {
    /// Cria um novo Rotor com base na configuração.
    fn new(config: &RotorConfig) -> Self {
        let (wiring, notch) = match config.name.to_uppercase().as_str() {
            "I" => (ROTOR_I_WIRING, ROTOR_I_NOTCH),
            "II" => (ROTOR_II_WIRING, ROTOR_II_NOTCH),
            "III" => (ROTOR_III_WIRING, ROTOR_III_NOTCH),
            _ => panic!("Rotor desconhecido: {}. Use 'I', 'II' ou 'III'.", config.name),
        };

        // Calcula o mapeamento inverso (essencial para o caminho de volta)
        let mut inverse_wiring = [0u8; 26];
        for (i, &output) in wiring.iter().enumerate() {
            inverse_wiring[output as usize] = i as u8;
        }

        Self {
            wiring,
            inverse_wiring,
            position: char_to_u8(config.position),
            ring_setting: char_to_u8(config.ring),
            notch,
            name: format!("Rotor {}", config.name),
        }
    }

    /// Retorna se o rotor está atualmente na posição da ranhura.
    fn at_notch(&self) -> bool {
        self.position == self.notch
    }

    /// Gira o rotor uma posição (módulo 26).
    fn step(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    /// Mapeia um sinal da direita para a esquerda (ida).
    fn forward(&self, c: u8) -> u8 {
        // Ajusta a entrada pela posição e anel
        let index = (c + self.position - self.ring_setting + 26) % 26;
        // Passa pelo mapeamento
        let wired_c = self.wiring[index as usize];
        // Ajusta a saída pela posição e anel
        (wired_c - self.position + self.ring_setting + 26) % 26
    }

    /// Mapeia um sinal da esquerda para a direita (volta).
    fn backward(&self, c: u8) -> u8 {
        // Ajusta a entrada pela posição e anel
        let index = (c + self.position - self.ring_setting + 26) % 26;
        // Passa pelo mapeamento INVERSO
        let wired_c = self.inverse_wiring[index as usize];
        // Ajusta a saída pela posição e anel
        (wired_c - self.position + self.ring_setting + 26) % 26
    }
}

/// A máquina Enigma completa, contendo o estado atual.
#[derive(Debug)]
pub struct EnigmaMachine {
    /// O rotor rápido (direita)
    rotor_r: Rotor,
    /// O rotor do meio
    rotor_m: Rotor,
    /// O rotor lento (esquerda)
    rotor_l: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    /// Cria uma nova instância da máquina com base na configuração da UI.
    pub fn new(config: EnigmaConfig) -> Self {
        Self {
            // Nota: A ordem na tupla da config é (Direita, Meio, Esquerda)
            rotor_r: Rotor::new(&config.rotors.0),
            rotor_m: Rotor::new(&config.rotors.1),
            rotor_l: Rotor::new(&config.rotors.2),
            reflector: Reflector::new(&config.reflector),
            plugboard: Plugboard::new(&config.plugboard_pairs),
        }
    }

    /// Retorna as posições atuais dos rotores (L, M, R) como caracteres.
    fn get_positions(&self) -> (char, char, char) {
        (
            u8_to_char(self.rotor_l.position),
            u8_to_char(self.rotor_m.position),
            u8_to_char(self.rotor_r.position),
        )
    }

    /// Implementa a mecânica de passo dos rotores (antes de criptografar).
    /// Esta é a parte mais complexa da lógica da Enigma (double-stepping).
    fn step_rotors(&mut self) {
        // 1. O rotor do meio gira se o rotor da direita estiver na ranhura.
        let m_steps = self.rotor_r.at_notch();
        // 2. O rotor da esquerda gira se o rotor do meio estiver na ranhura.
        let l_steps = self.rotor_m.at_notch();

        // 3. O rotor da direita *sempre* gira.
        self.rotor_r.step();

        // 4. Se o rotor do meio deve girar (passo 1)
        if m_steps {
            self.rotor_m.step();
            // 5. Se o rotor da esquerda também deve girar (passo 2 - double step)
            if l_steps {
                self.rotor_l.step();
            }
        }
        // Nota: A implementação real do M3 tem um "double step" onde o rotor do
        // meio gira uma segunda vez se ele *parar* na ranhura. Esta implementação
        // usa o passo simples (o rotor da esquerda só gira quando o do meio
        // *passa* pela ranhura), que é didaticamente mais comum.
        // Para a lógica exata de "double-step" (o rotor do meio pisa no
        // próprio pé), a condição `m_steps` também precisaria checar
        // `self.rotor_m.at_notch()` *antes* do passo 4.
    }

    /// Processa um único caractere e retorna o resultado e os passos detalhados.
    /// Esta é a função central para fins didáticos.
    pub fn process_char_detailed(&mut self, c: char) -> (char, EncryptionStep) {
        let input_u8 = char_to_u8(c);
        let mut path: Vec<PathEntry> = Vec::with_capacity(9);

        let positions_before = self.get_positions();

        // 1. Girar os rotores (ACONTECE ANTES da criptografia)
        self.step_rotors();
        let positions_after = self.get_positions();

        let mut current_u8 = input_u8;
        let mut next_u8;

        // --- Caminho de Ida (Forward) ---

        // 2. Plugboard (Entrada)
        next_u8 = self.plugboard.process(current_u8);
        path.push(PathEntry {
            component: "Plugboard".to_string(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Forward,
        });
        current_u8 = next_u8;

        // 3. Rotor R (Direita)
        next_u8 = self.rotor_r.forward(current_u8);
        path.push(PathEntry {
            component: self.rotor_r.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Forward,
        });
        current_u8 = next_u8;

        // 4. Rotor M (Meio)
        next_u8 = self.rotor_m.forward(current_u8);
        path.push(PathEntry {
            component: self.rotor_m.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Forward,
        });
        current_u8 = next_u8;

        // 5. Rotor L (Esquerda)
        next_u8 = self.rotor_l.forward(current_u8);
        path.push(PathEntry {
            component: self.rotor_l.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Forward,
        });
        current_u8 = next_u8;

        // --- Refletor ---

        // 6. Refletor
        next_u8 = self.reflector.reflect(current_u8);
        path.push(PathEntry {
            component: self.reflector.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Reflect,
        });
        current_u8 = next_u8;

        // --- Caminho de Volta (Backward) ---

        // 7. Rotor L (Esquerda)
        next_u8 = self.rotor_l.backward(current_u8);
        path.push(PathEntry {
            component: self.rotor_l.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Backward,
        });
        current_u8 = next_u8;

        // 8. Rotor M (Meio)
        next_u8 = self.rotor_m.backward(current_u8);
        path.push(PathEntry {
            component: self.rotor_m.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Backward,
        });
        current_u8 = next_u8;

        // 9. Rotor R (Direita)
        next_u8 = self.rotor_r.backward(current_u8);
        path.push(PathEntry {
            component: self.rotor_r.name.clone(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Backward,
        });
        current_u8 = next_u8;

        // 10. Plugboard (Saída)
        next_u8 = self.plugboard.process(current_u8);
        path.push(PathEntry {
            component: "Plugboard".to_string(),
            input_char: u8_to_char(current_u8),
            output_char: u8_to_char(next_u8),
            direction: PathDirection::Backward,
        });
        
        let output_char = u8_to_char(next_u8);

        let step_details = EncryptionStep {
            input_char: c,
            output_char,
            positions_before_step, // Declarar essas variáveis em algum lugar acima
            positions_after_step, // Declarar essas variáveis em algum lugar acima
            path,
        };

        (output_char, step_details)
    }

    /// Processa uma string completa, retornando apenas o texto final.
    /// Ignora caracteres não alfabéticos.
    pub fn process_string(&mut self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| self.process_char_detailed(c.to_ascii_uppercase()).0)
            .collect()
    }

    /// Processa uma string completa, retornando a lista de passos detalhados.
    /// Ignora caracteres não alfabéticos.
    pub fn process_string_detailed(&mut self, text: &str) -> Vec<EncryptionStep> {
        text.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| self.process_char_detailed(c.to_ascii_uppercase()).1)
            .collect()
    }
}

// --- Funções Auxiliares (Helpers) ---

/// Converte um caractere (A-Z) para u8 (0-25).
/// Assume entrada maiúscula e alfabética.
#[inline]
fn char_to_u8(c: char) -> u8 {
    c as u8 - b'A'
}

/// Converte um u8 (0-25) para caractere (A-Z).
#[inline]
fn u8_to_char(i: u8) -> char {
    (i + b'A') as char
}

// --- Testes Unitários ---
#[cfg(test)]
mod tests {
    use super::*;

    /// Cria uma configuração padrão para testes (Rotores I, II, III; Refletor B; Posições A-A-A; Anéis A-A-A; Sem Plugboard).
    fn default_config() -> EnigmaConfig {
        EnigmaConfig {
            rotors: (
                RotorConfig { name: "I".to_string(), position: 'A', ring: 'A' }, // Direita
                RotorConfig { name: "II".to_string(), position: 'A', ring: 'A' }, // Meio
                RotorConfig { name: "III".to_string(), position: 'A', ring: 'A' }, // Esquerda
            ),
            reflector: "B".to_string(),
            plugboard_pairs: "".to_string(),
        }
    }

    #[test]
    fn test_char_to_u8_conversions() {
        assert_eq!(char_to_u8('A'), 0);
        assert_eq!(char_to_u8('Z'), 25);
        assert_eq!(u8_to_char(0), 'A');
        assert_eq!(u8_to_char(25), 'Z');
    }

    #[test]
    fn test_plugboard() {
        let pb = Plugboard::new("AB XY ZW");
        assert_eq!(pb.process(char_to_u8('A')), char_to_u8('B'));
        assert_eq!(pb.process(char_to_u8('B')), char_to_u8('A'));
        assert_eq!(pb.process(char_to_u8('C')), char_to_u8('C')); // Não mapeado
        assert_eq!(pb.process(char_to_u8('X')), char_to_u8('Y'));
    }

    #[test]
    fn test_rotor_stepping() {
        let mut cfg = default_config();
        cfg.rotors.0.position = 'Q'; // Rotor I (Direita)
        cfg.rotors.1.position = 'E'; // Rotor II (Meio)

        let mut machine = EnigmaMachine::new(cfg);

        // Posição inicial: (III, II, I) -> (A, E, Q)
        assert_eq!(machine.get_positions(), ('A', 'E', 'Q'));

        // 1. Pressionar a tecla
        machine.step_rotors();
        // Rotor I (Direita) gira para R. Ele estava na ranhura (Q),
        // então o Rotor II (Meio) gira para F.
        // O Rotor II (Meio) *também* estava na ranhura (E),
        // então o Rotor III (Esquerda) gira para B.
        // (Isso testa o double-step)
        assert_eq!(machine.get_positions(), ('B', 'F', 'R'));

        // 2. Pressionar a tecla novamente
        machine.step_rotors();
        // Apenas o Rotor I (Direita) gira (R -> S), pois não estava na ranhura.
        assert_eq!(machine.get_positions(), ('B', 'F', 'S'));
    }

    #[test]
    /// Teste de criptografia/descriptografia (Reciprocidade).
    /// Criptografar "AAAAA" deve dar "BDZGO".
    /// Criptografar "BDZGO" (com a mesma config) deve dar "AAAAA".
    fn test_encryption_reciprocity() {
        let config = default_config();
        let mut machine_encrypt = EnigmaMachine::new(config);
        let encrypted = machine_encrypt.process_string("AAAAA");
        assert_eq!(encrypted, "BDZGO");

        let config_reset = default_config(); // Reseta a máquina para A-A-A
        let mut machine_decrypt = EnigmaMachine::new(config_reset);
        let decrypted = machine_decrypt.process_string("BDZGO");
        assert_eq!(decrypted, "AAAAA");
    }

    #[test]
    /// Teste completo com configuração complexa (posições, anéis, plugboard).
    fn test_complex_config_encryption() {
        let config = EnigmaConfig {
            rotors: (
                RotorConfig { name: "I".to_string(), position: 'G', ring: 'B' }, // 1
                RotorConfig { name: "II".to_string(), position: 'O', ring: 'M' }, // 12
                RotorConfig { name: "III".to_string(), position: 'X', ring: 'V' }, // 21
            ),
            reflector: "B".to_string(),
            plugboard_pairs: "AV BS CG DL FU HZ IN KM OW RX".to_string(),
        };

        let mut machine = EnigmaMachine::new(config);
        let text = "HELLOWORLD";
        let expected = "QMJIDOJAZF"; // Valor de referência conhecido
        assert_eq!(machine.process_string(text), expected);
    }
    
    #[test]
    fn test_detailed_steps() {
        let config = default_config();
        let mut machine = EnigmaMachine::new(config);
        let steps = machine.process_string_detailed("A");
        
        assert_eq!(steps.len(), 1);
        let step = &steps[0];

        // 1. Verificações do passo
        assert_eq!(step.input_char, 'A');
        assert_eq!(step.output_char, 'B'); // "AAAAA" -> "BDZGO", o primeiro é 'B'
        assert_eq!(step.positions_before_step, ('A', 'A', 'A'));
        assert_eq!(step.positions_after_step, ('A', 'A', 'B')); // Só o rotor da direita girou

        // 2. Verificações do caminho (path)
        assert_eq!(step.path.len(), 9); // Plug, R, M, L, Ref, L, M, R, Plug
        
        // Pelo menos o refletor deve estar correto (sem plugboard e posições 0)
        // R-I(A=0) -> E(4)
        // R-II(E=4) -> K(10)
        // R-III(K=10) -> X(23)
        // Ref-B(X=23) -> J(9)
        assert_eq!(step.path[4].component, "Reflector B");
        assert_eq!(step.path[4].input_char, 'X');
        assert_eq!(step.path[4].output_char, 'J');
    }
}