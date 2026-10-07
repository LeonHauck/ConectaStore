use megastore::graph::{Graph, NodeType};

#[test]
fn test_add_and_retrieve_product() {
    let mut g = Graph::new();
    g.add_node(1, "Cadeira".to_string(), NodeType::Product);
    
    let node = g.get_node(1).unwrap();
    assert_eq!(node.name, "Cadeira");
    assert_eq!(node.node_type, NodeType::Product);
}

#[test]
fn test_recommendation_filtering() {
    let mut g = Graph::new();
    g.add_node(1, "Prod A".to_string(), NodeType::Product);
    g.add_node(2, "Prod B".to_string(), NodeType::Product);
    g.add_node(3, "Prod C".to_string(), NodeType::Product);
    g.add_node(101, "C1".to_string(), NodeType::Customer);
    g.add_node(102, "C2".to_string(), NodeType::Customer);

    g.add_purchase(101, 1);
    g.add_purchase(102, 1);
    g.add_purchase(102, 2);

    let recs = g.recommend_products(101, 5);
    
    // C1 should be recommended Prod B because C2 bought both A and B.
    // C1 should NOT be recommended Prod A (already bought).
    // Prod C has no connections, should not be recommended.
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].id, 2);
}

#[test]
fn test_no_duplicate_recommendations() {
    let mut g = Graph::new();
    g.add_node(1, "P1".to_string(), NodeType::Product);
    g.add_node(2, "P2".to_string(), NodeType::Product);
    
    g.add_node(101, "C1".to_string(), NodeType::Customer);
    g.add_node(102, "C2".to_string(), NodeType::Customer);
    g.add_node(103, "C3".to_string(), NodeType::Customer);

    g.add_purchase(101, 1);
    
    // Both C2 and C3 bought P1 and P2
    g.add_purchase(102, 1);
    g.add_purchase(102, 2);
    
    g.add_purchase(103, 1);
    g.add_purchase(103, 2);

    // Even though multiple paths lead to P2, it should only appear once in recommendations
    let recs = g.recommend_products(101, 5);
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].id, 2);
}
