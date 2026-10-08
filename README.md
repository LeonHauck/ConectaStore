<h1 align="center">
  <img alt="ConectaStore Logo" src="https://img.shields.io/badge/ConectaStore-000000?style=for-the-badge&logo=rust&logoColor=white" />
</h1>

<p align="center">
  <strong>Sistema de Recomendação de Produtos baseado em Grafos para E-commerce</strong>
</p>

<p align="center">
  <a href="#-sobre-o-projeto">Sobre</a> •
  <a href="#-arquitetura-e-estruturas-de-dados">Arquitetura</a> •
  <a href="#-como-executar">Como Executar</a> •
  <a href="#-testes-e-desempenho">Testes</a> •
  <a href="#-vídeo-pitch">Vídeo Pitch</a>
</p>

---

## 🎯 Sobre o Projeto

O projeto **ConectaStore** é uma solução de alto desempenho desenvolvida em **Rust** para a *MegaStore*. Seu principal objetivo é resolver a estagnação de vendas causada por recomendações genéricas de produtos.

Em vez de focar apenas em "itens mais vendidos", o sistema mapeia o histórico e o comportamento de compras utilizando a teoria dos grafos. Isso nos permite criar um **Filtro Colaborativo**, inferindo gostos em comum entre diferentes clientes e recomendando novos produtos de forma altamente personalizada e assertiva.

## 🏗 Arquitetura e Estruturas de Dados

A base teórica e prática do projeto se fundamenta na representação das interações dos usuários como um **Grafo Bidirecional Não-Ponderado**.

- **Vértices (Nós):** Clientes e Produtos.
- **Arestas (Conexões):** Histórico de compras (Cliente $\leftrightarrow$ Produto).

Para garantir escalabilidade e eficiência de memória no catálogo de milhões de itens da MegaStore, evitamos o uso de Matrizes de Adjacência. O projeto foi implementado utilizando:

- `HashMap<u32, Vec<u32>>`: Para construir a **Lista de Adjacência**, garantindo acesso em tempo $O(1)$ aos nós vizinhos e mantendo o consumo de memória estritamente proporcional aos dados reais de conexões ($O(V+E)$).
- `VecDeque`: Estrutura de fila utilizada para executar a **Busca em Largura (BFS)** de forma otimizada, guiando a descoberta de produtos similares.
- `HashSet`: Utilizado estrategicamente no algoritmo para prevenir ciclos e impedir recomendações duplicadas ou de produtos que o usuário já adquiriu, sempre com checagens $O(1)$.

## 🚀 Como Executar

**Pré-requisitos:** Você precisará ter o compilador do [Rust e o Cargo](https://rustup.rs/) instalados na sua máquina.

1. Clone este repositório:
   ```bash
   git clone https://github.com/LeonHauck/ConectaStore.git
   ```
2. Acesse a pasta do projeto no seu terminal:
   ```bash
   cd ConectaStore
   ```
3. Compile e execute a demonstração do motor de recomendação:
   ```bash
   cargo run
   ```

## 🧪 Testes e Desempenho

O sistema conta com baterias de testes unitários e de integração (presentes em `tests/integration_tests.rs`). Os testes cobrem:
- Criação e integridade dos nós e conexões.
- Prevenção de duplicidade em caminhos cíclicos.
- Lógica do Filtro Colaborativo (não recomendar produtos já possuídos).

**Para executar os testes automatizados:**
```bash
cargo test
```

> **Performance:** Devido ao controle de limite de profundidade (Depth = 3) na Busca em Largura e uso extensivo de tabelas Hash, as medições apontam a resolução de consultas na casa dos milissegundos (`< 1ms`), independentemente do tamanho total do catálogo, escalando o custo operacional apenas em relação ao grau ativo do usuário.

## 📹 Vídeo Pitch

Apresentação da solução, da modelagem matemática adotada e demonstração prática da execução e dos testes:

🔗 **https://youtu.be/i8VOKBIjWvQ**
