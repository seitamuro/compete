#![allow(dead_code)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        road: [(usize, usize);n-1],
        query: [(usize, usize);q]
    }

    let mut path = vec![vec![]; n];
    for (i, j) in road.iter() {
        path[*i-1].push(*j-1);
        path[*j-1].push(*i-1);
    }

    let mut dist = vec![None; n];
    let mut todo = vec![];
    dist[0] = Some(0);
    for i in path[0].iter() {
        let i = *i;
        todo.push(i);
        if dist[i] == None {
            dist[i] = Some(1);
        }
    }

    while !todo.is_empty() {
        let i = todo.remove(0);
        let state = match dist[i] {
            Some(x) => if x == 0 { Some(1) } else { Some(0) },
            None => panic!("unexpected"),
        };

        for j in path[i].iter() {
            let j = *j;
            if dist[j] == None {
                dist[j] = state;
                todo.push(j);
            }
        }
    }

    for (i, j) in query.iter() {
        if dist[*i-1] == dist[*j-1] {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}

/// path: path to the other nodes
/// cost: weight of the path corresponding to each path
///// https://qiita.com/okaryo/items/8e6cd73f8a676b7a5d75
//fn warshall_floyd(path: Vec<Vec<usize>>, cost: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
//    let mut table = vec![vec![usize::MAX; path.len()]; path.len()];
//
//    for i in 0..path.len() {
//        table[i][i] = 0;
//        for (j, c) in path[i].iter().zip(cost[i].iter()) {
//            table[i][*j] = *c;
//        }
//    }
//
//    for k in 0..path.len() {
//        for i in 0..path.len() {
//            for j in 0..path.len() {
//                table[i][j] = table[i][j].min(table[i][k].saturating_add(table[k][j]));
//            }
//        }
//    }
//
//    table
//}
//
fn bfs(start: usize, node: &[usize], path: &[Vec<usize>]) -> Vec<Option<usize>> {
    let mut ans = vec![None; node.len()];
    let mut todo = Vec::<usize>::new();

    ans[start] = Some(0);
    for &i in path[start].iter() {
        if ans[i] == None {
            let x = ans[start].unwrap();
            todo.push(i);
            ans[i] = Some(x + 1);
        }
    }

    while !todo.is_empty() {
        let i = todo.remove(0);

        for &node in path[i].iter() {
            if ans[node] == None {
                let x = ans[i].unwrap();
                todo.push(node);
                ans[node] = Some(x + 1);
            }
        }
    }

    ans
}
