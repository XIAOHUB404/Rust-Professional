use std::collections::{HashMap, HashSet};
use std::fs;

pub fn count_provinces() -> String {
    // 读取JSON文件
    let content = fs::read_to_string("district.json").expect("Failed to read file");
    let data: HashMap<String, HashMap<String, Vec<String>>> = serde_json::from_str(&content).expect("Failed to parse JSON");
    
    // 按批次处理并计算连通分量
    let mut results = Vec::new();
    let mut keys: Vec<_> = data.keys().collect();
    keys.sort(); // 确保按批次顺序处理
    
    for key in keys {
        if let Some(cities) = data.get(key) {
            // 合并同一城市的所有邻居，包括重复出现的城市
            let mut merged_cities: HashMap<String, HashSet<String>> = HashMap::new();
            for (city, neighbors) in cities {
                // 确保城市和它的邻居都被添加到映射中
                for neighbor in neighbors {
                    // 跳过自环
                    if city == neighbor {
                        continue;
                    }
                    merged_cities.entry(city.clone())
                        .or_insert_with(HashSet::new)
                        .insert(neighbor.clone());
                    
                    // 双向添加邻居关系
                    merged_cities.entry(neighbor.clone())
                        .or_insert_with(HashSet::new)
                        .insert(city.clone());
                }
            }
            
            // 收集所有城市
            let mut all_cities = HashSet::new();
            for (city, neighbors) in &merged_cities {
                all_cities.insert(city.clone());
                all_cities.extend(neighbors.iter().cloned());
            }
            
            // 创建并查集
            let mut parent: HashMap<String, String> = HashMap::new();
            let mut rank: HashMap<String, usize> = HashMap::new();
            for city in &all_cities {
                parent.insert(city.clone(), city.clone());
                rank.insert(city.clone(), 0);
            }
            
            // 合并连通的城市
            for (city, neighbors) in &merged_cities {
                for neighbor in neighbors {
                    if city != neighbor {
                        union(&mut parent, &mut rank, city, neighbor);
                    }
                }
            }
            
            // 计算连通分量数量
            let mut roots = HashSet::new();
            for city in all_cities {
                roots.insert(find(&mut parent, &city));
            }
            
            results.push(roots.len().to_string());
        }
    }
    
    results.join(",")
}

// 查找城市所属的连通分量的根节点，使用路径压缩
fn find(parent: &mut HashMap<String, String>, city: &str) -> String {
    let mut current = city.to_string();
    let mut path = Vec::new();
    
    // 找到根节点
    while current != parent[&current] {
        path.push(current.clone());
        current = parent[&current].clone();
    }
    
    // 路径压缩：将路径上的所有节点直接连接到根节点
    for node in path {
        parent.insert(node, current.clone());
    }
    
    current
}

// 按秩合并两个城市所在的连通分量
fn union(parent: &mut HashMap<String, String>, rank: &mut HashMap<String, usize>, city1: &str, city2: &str) {
    let root1 = find(parent, city1);
    let root2 = find(parent, city2);
    
    if root1 != root2 {
        let rank1 = rank[&root1];
        let rank2 = rank[&root2];
        
        if rank1 < rank2 {
            parent.insert(root1.clone(), root2);
        } else if rank1 > rank2 {
            parent.insert(root2.clone(), root1);
        } else {
            parent.insert(root2.clone(), root1.clone());
            rank.insert(root1, rank1 + 1);
        }
    }
}