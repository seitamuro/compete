use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        road: [(usize, usize);n-1],
        query: [(usize, usize);q]
    }

    let mut path = vec![vec![];n];
    let mut cost = vec![vec![];n];
    let road: Vec<(usize, usize)> = road.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();
    let query: Vec<(usize, usize)> = query.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();
    for (r1, r2) in road.into_iter() {
        path[r1].push(r2);
        path[r2].push(r1);
        cost[r1].push(1);
        cost[r2].push(1);
    }

    let mut dist = Vec::new();
    for i in 0..n {
        let node: Vec<usize> = (0..n).collect();
        dist.push(bfs(i, &node, &path));
    }

    for (from, to) in query.iter() {
        if dist[*from][*to].unwrap() % 2 == 1 {
            println!("Road");
        } else {
            println!("Town");
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
