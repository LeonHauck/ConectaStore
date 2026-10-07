use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    Customer,
    Product,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub node_type: NodeType,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<u32, Node>,
    pub adj_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            adj_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: u32, name: String, node_type: NodeType) {
        self.nodes.insert(id, Node { id, name, node_type });
        self.adj_list.entry(id).or_insert(Vec::new());
    }

    pub fn add_purchase(&mut self, customer_id: u32, product_id: u32) {
        if self.nodes.contains_key(&customer_id) && self.nodes.contains_key(&product_id) {
            self.adj_list.get_mut(&customer_id).unwrap().push(product_id);
            self.adj_list.get_mut(&product_id).unwrap().push(customer_id);
        }
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn recommend_products(&self, customer_id: u32, limit: usize) -> Vec<Node> {
        let mut recommendations = Vec::new();
        
        let customer = match self.nodes.get(&customer_id) {
            Some(n) if n.node_type == NodeType::Customer => n,
            _ => return recommendations,
        };

        // Fila armazena tuplas de (id_do_no, profundidade)
        let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
        let mut visited: HashSet<u32> = HashSet::new();
        let mut product_scores: HashMap<u32, u32> = HashMap::new();

        let mut already_bought = HashSet::new();
        if let Some(neighbors) = self.adj_list.get(&customer_id) {
            for &prod_id in neighbors {
                already_bought.insert(prod_id);
                queue.push_back((prod_id, 1)); // Profundidade 1: Produtos do cliente
                visited.insert(prod_id);
            }
        }
        
        visited.insert(customer_id);

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth >= 3 {
                // Limita a busca a profundidade 3 (Clientes similares e os produtos deles)
                continue;
            }

            if let Some(neighbors) = self.adj_list.get(&current_id) {
                for &neighbor_id in neighbors {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id);
                        queue.push_back((neighbor_id, depth + 1));
                    }
                    
                    if depth == 2 {
                        // Estamos analisando outro cliente, os vizinhos dele são produtos (Profundidade 3)
                        if let Some(node) = self.nodes.get(&neighbor_id) {
                            if node.node_type == NodeType::Product && !already_bought.contains(&neighbor_id) {
                                *product_scores.entry(neighbor_id).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut scored_products: Vec<(u32, u32)> = product_scores.into_iter().collect();
        // Ordenar de forma decrescente pela pontuação (quantidade de vezes encontrado)
        scored_products.sort_by(|a, b| b.1.cmp(&a.1));

        for (prod_id, _) in scored_products.into_iter().take(limit) {
            if let Some(node) = self.nodes.get(&prod_id) {
                recommendations.push(node.clone());
            }
        }

        recommendations
    }
}
