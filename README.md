# Lab - Processamento de Dados JSON com Rust

Um projeto simples em Rust que demonstra como processar dados JSON usando a biblioteca `serde_json`.

## 📋 Sobre o Projeto

Este projeto lê dados de uma pessoa em formato JSON e extrai informações específicas como nome e endereço, utilizando a biblioteca `serde_json` para parsing.

## 🚀 Tecnologias Utilizadas

- **Rust** - Linguagem de programação
- **serde_json** - Biblioteca para serialização/deserialização JSON

## 📁 Estrutura do Projeto

```
lab/
├── src/
│   ├── main.rs              # Código principal
│   └── serde_json/
│       └── data.json        # Dados de exemplo
├── Cargo.toml               # Configuração do projeto
└── README.md                # Este arquivo
```

## 🔧 Como Executar

1. **Pré-requisitos**: Certifique-se de ter o Rust instalado
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone o repositório**:
   ```bash
   git clone <https://github.com/DanielDeAzevedoCordeiro1/manipulando-json-rust>
   cd lab
   ```

3. **Execute o projeto**:
   ```bash
   cargo run
   ```

## 📊 Dados de Exemplo

O projeto utiliza dados de exemplo de uma pessoa com as seguintes informações:
- Nome completo
- Idade, email e telefone
- Profissão e salário
- Endereço completo (rua, CEP, bairro)

## 🎯 Funcionalidades

- ✅ Leitura de arquivo JSON
- ✅ Parsing de dados JSON
- ✅ Extração de campos específicos
- ✅ Acesso a objetos aninhados (endereço)
- ✅ Debug de informações extraídas

## 📈 Exemplo de Saída

```
[src/main.rs:12] nome = String("Ana Silva")
[src/main.rs:12] rua = "Rua das Flores, 123"
[src/main.rs:12] cep = "01234-567"
[src/main.rs:12] bairro = "Centro"
```
