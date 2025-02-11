use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn count_provinces() -> String {
    let data = fs::read_to_string("district.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut result = Vec::new();

    if let Value::Object(batches) = json {
        for i in 1..=batches.len() {
            let batch_key = i.to_string();
            if let Some(Value::Object(cities)) = batches.get(&batch_key) {
                let province_count = count_connected_groups(cities);
                result.push(province_count);
            }
        }
    }

    format!(
        "{}",
        result
            .iter()
            .map(|count| count.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn count_connected_groups(cities: &serde_json::Map<String, Value>) -> i32 {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_cities = HashSet::new();

    // 收集所有城市（包括连接中提到的城市）
    for (city, connected_cities) in cities {
        all_cities.insert(city.clone());
        if let Value::Array(city_list) = connected_cities {
            for connected_city in city_list.iter().filter_map(|v| v.as_str()) {
                all_cities.insert(connected_city.to_string());
            }
        }
    }

    // 构建连接关系
    for (city, connected_cities) in cities {
        if let Value::Array(city_list) = connected_cities {
            let connections: HashSet<String> = city_list
                .iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect();

            // 添加双向连接
            for connected_city in &connections {
                graph
                    .entry(city.clone())
                    .or_insert_with(HashSet::new)
                    .insert(connected_city.clone());
                graph
                    .entry(connected_city.clone())
                    .or_insert_with(HashSet::new)
                    .insert(city.clone());
            }
        }
    }

    // 确保所有城市都在图中，即使它们没有连接
    for city in &all_cities {
        graph.entry(city.clone()).or_insert_with(HashSet::new);
    }

    let mut visited = HashSet::new();
    let mut provinces = 0;

    // DFS遍历
    for city in all_cities {
        if !visited.contains(&city) {
            dfs(&graph, &city, &mut visited);
            provinces += 1;
        }
    }

    provinces
}

fn dfs(graph: &HashMap<String, HashSet<String>>, city: &str, visited: &mut HashSet<String>) {
    visited.insert(city.to_string());
    if let Some(neighbors) = graph.get(city) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(graph, neighbor, visited);
            }
        }
    }
}
