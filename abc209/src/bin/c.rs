use proconio::{input};

fn main() {
    input! {
        n: usize,
        q: usize,
        road: [(usize, usize);n-1],
        query: [(usize, usize);1]
    }

    let mut path = vec![vec![];n];
    let mut cost = vec![vec![];n];
    let road: Vec<(usize, usize)> = road.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();
    for (r1, r2) in road.into_iter() {
        path[r1].push(r2);
        path[r2].push(r1);
        cost[r1].push(1);
        cost[r2].push(1);
    }

    let dist = warshall_floyd(path, cost);

    for 
}

/// path: path to the other nodes
/// cost: weight of the path corresponding to each path
/// https://qiita.com/okaryo/items/8e6cd73f8a676b7a5d75
fn warshall_floyd(path: Vec<Vec<usize>>, cost: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut table = vec![vec![usize::MAX; path.len()]; path.len()];

    for i in 0..path.len() {
        table[i][i] = 0;
        for (j, c) in path[i].iter().zip(cost[i].iter()) {
            table[i][*j] = *c;
        }
    }

    for k in 0..path.len() {
        for i in 0..path.len() {
            for j in 0..path.len() {
                println!("{} -> {} | {} | {} {}", i, j, table[i][j], table[i][k], table[k][j]);
                table[i][j] = table[i][j].min(table[i][k].saturating_add(table[k][j]));
            }
        }
    }

    table
}
