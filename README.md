# ConectaStore - Sistema de Recomendação de Produtos

O projeto **ConectaStore** é uma solução desenvolvida em Rust para a MegaStore com o objetivo de gerar recomendações relevantes para os clientes com base no histórico de compras através da estruturação dos dados como um grafo.

## Objetivo e Funcionamento do Sistema
Este sistema visa mitigar o problema de recomendações genéricas de um e-commerce. Ao invés de apenas recomendar "os mais vendidos", o sistema mapeia a jornada de clientes e produtos através de um **Grafo Bidirecional**. 
A recomendação funciona através de um Filtro Colaborativo usando o algoritmo de **Busca em Largura (BFS - Breadth-First Search)**. Quando pedimos uma recomendação para um usuário, o sistema vasculha:
1. Os produtos que o usuário comprou.
2. Outros usuários que compraram os mesmos produtos.
3. Novos produtos comprados por esses outros usuários, sugerindo-os.

## Tecnologias e Estruturas Utilizadas
- **Linguagem:** Rust (Devido ao seu alto desempenho e forte controle sobre a memória).
- **Estruturas de Dados:**
  - `HashMap`: Utilizado tanto para armazenar os nós do grafo por ID permitindo busca instantânea ($O(1)$), quanto para construir a **Lista de Adjacência** que sustenta as arestas.
  - `VecDeque`: Estrutura de fila para executar a BFS na recomendação.
  - `HashSet`: Para prevenir loops em conexões cíclicas e garantir a não duplicação de recomendações, bloqueando também a indicação de itens já comprados.
  - `Vec` e Algoritmos de ordenação: Para construir e ordenar os top-N produtos recomendados por relevância.

## Arquitetura da Solução
A solução está dividida nos seguintes módulos:
- `src/graph.rs`: Núcleo da lógica. Contém os modelos de `Node`, `NodeType` e a estrutura do `Graph` junto com seus métodos de inserção e travessia (BFS).
- `src/lib.rs`: Expõe o módulo para testes e importação.
- `src/main.rs`: Inicializa o projeto, cadastra produtos e clientes mockados na memória, injeta compras (arestas) e roda um benchmark inicial para medir tempo.
- `tests/integration_tests.rs`: Bateria de testes automatizados garantindo que as lógicas de travessia estão corretas.

## Instruções para Compilação e Execução
Certifique-se de ter o [Rust e o Cargo instalados](https://www.rust-lang.org/tools/install).
Para compilar e executar o projeto:
```bash
# Na pasta raiz do projeto:
cargo run
```

## Instruções para Execução dos Testes
Testes de unidade e integração foram implementados garantindo a correta adição de nós, arestas e geração e deduplicação de recomendações.
Para rodá-los:
```bash
cargo test
```

## Resultados dos Testes de Desempenho
As medições básicas (incluídas na função `main`) revelam que a inserção de produtos e a execução da BFS local num limite de profundidade (3-steps) tomam menos de 1 milissegundo (`< 1ms`) em pequenas/médias escalas. Ao limitar a BFS apenas à "vizinhança de interesses", a complexidade de busca ignora a esmagadora maioria do catálogo, tornando a ferramenta ultra performática para consultas instantâneas em cenários de alta escalabilidade.

## Vídeo Pitch
*(Insira o link do YouTube para o seu Vídeo Pitch aqui após gravar e publicar)*
