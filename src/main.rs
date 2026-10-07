use megastore::graph::{Graph, NodeType};
use std::time::Instant;

fn main() {
    println!("=== ConectaStore: Sistema de Recomendação Baseado em Grafos ===");
    let mut g = Graph::new();

    println!("Carregando dados (Produtos e Clientes)...");
    let start_load = Instant::now();

    // Cadastrando Produtos
    g.add_node(1, "Smartphone".to_string(), NodeType::Product);
    g.add_node(2, "Capa de Celular".to_string(), NodeType::Product);
    g.add_node(3, "Fone de Ouvido Bluetooth".to_string(), NodeType::Product);
    g.add_node(4, "Smartwatch".to_string(), NodeType::Product);
    g.add_node(5, "Notebook".to_string(), NodeType::Product);
    g.add_node(6, "Mouse Sem Fio".to_string(), NodeType::Product);
    g.add_node(7, "Livro: Rust Programming".to_string(), NodeType::Product);

    // Cadastrando Clientes
    g.add_node(101, "Alice".to_string(), NodeType::Customer);
    g.add_node(102, "Bruno".to_string(), NodeType::Customer);
    g.add_node(103, "Carla".to_string(), NodeType::Customer);
    g.add_node(104, "Diego".to_string(), NodeType::Customer);

    // Adicionando Histórico de Compras (Arestas)
    // Alice comprou Smartphone e Capa
    g.add_purchase(101, 1);
    g.add_purchase(101, 2);

    // Bruno comprou Smartphone, Fone de Ouvido e Smartwatch
    g.add_purchase(102, 1);
    g.add_purchase(102, 3);
    g.add_purchase(102, 4);

    // Carla comprou Fone de Ouvido e Livro
    g.add_purchase(103, 3);
    g.add_purchase(103, 7);

    // Diego comprou Notebook e Mouse Sem Fio
    g.add_purchase(104, 5);
    g.add_purchase(104, 6);

    let load_duration = start_load.elapsed();
    println!("Dados carregados em: {:?}", load_duration);

    // Exemplo de Recomendação
    println!("\n--- Gerando recomendações para Alice (ID 101) ---");
    println!("(Alice comprou: Smartphone, Capa de Celular)");
    
    let start_rec1 = Instant::now();
    let recs_alice = g.recommend_products(101, 3);
    let rec_duration1 = start_rec1.elapsed();
    
    if recs_alice.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (i, p) in recs_alice.iter().enumerate() {
            println!("{}. {} (ID: {})", i + 1, p.name, p.id);
        }
    }
    println!("Tempo da operação: {:?}", rec_duration1);

    println!("\n--- Gerando recomendações para Carla (ID 103) ---");
    println!("(Carla comprou: Fone de Ouvido, Livro)");
    
    let start_rec2 = Instant::now();
    let recs_carla = g.recommend_products(103, 3);
    let rec_duration2 = start_rec2.elapsed();
    
    if recs_carla.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (i, p) in recs_carla.iter().enumerate() {
            println!("{}. {} (ID: {})", i + 1, p.name, p.id);
        }
    }
    println!("Tempo da operação: {:?}", rec_duration2);
}
