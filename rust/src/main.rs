use std::collections::VecDeque;

use proconio::input;


fn main() {
    get_time();
    let input = Input::new();
    let output = solve(&input);
    println!("{}", output.m);
    for (a, b, c, d) in output.vec {
        println!("{} {} {} {}", a, b, c, d);
    }
}

enum NodeType {
    Normal,
    Relay,
}

// 有効かつ一番近い奴を親として木を作る
fn solve(input: &Input) -> Output {
    let n = input.n;
    let mut s = vec![(0, 0); 1];
    let mut nodes = vec![(0, 0); 1];
    let mut edges = Vec::<(usize, usize, usize, usize)>::new();
    let mut parents = vec![0usize; input.n+1];
    let mut children: Vec<Vec<usize>> = vec![vec![]; input.n+1];
    for ab in input.ab.iter() {
        nodes.push(*ab);
    }

    for (i, &(x1, y1)) in nodes.iter().enumerate() { // children
        if x1 == 0 && y1 == 0 {
            continue;
        }

        let mut dist_min = std::i32::MAX;
        for (j, &(x2, y2)) in nodes.iter().enumerate() { // parent candidate
            if i == j {
                continue;
            }
            if x2 > x1 || y2 > y1 {
                continue;
            }

            let dist = (x1 - x2).abs() + (y1 - y2).abs();
            if dist < dist_min {
                dist_min = dist;
                parents[i] = j;
            }
        }

        children[parents[i]].push(i);
    }

    let mut visited = vec![false; n+1];
    let mut ans_v = vec![];
    let mut queue = VecDeque::<usize>::new();
    queue.push_back(0);

    while let Some(u) = queue.pop_front() {
        if visited[u] {
            continue;
        }
        visited[u] = true;

        if children[u].is_empty() {
            continue;
        }

        // 子が1このばあい
        if children[u].len() == 1 {
            let v = children[u][0];
            let (x1, y1) = nodes[u];
            let (x2, y2) = nodes[v];
            ans_v.push((x1, y1, x2, y2));
            edges.push((x1 as usize, y1 as usize, x2 as usize, y2 as usize));
            queue.push_back(v);
            continue;
        }

        // 子が２個以上の場合
        let mut left_min = children[u].iter().map(|&v| nodes[v].0).min().unwrap();
        let mut left_min_idx = children[u].iter().position(|&v| nodes[v].0 == left_min).unwrap();
        let mut right_max = children[u].iter().map(|&v| nodes[v].0).max().unwrap();
        let mut right_max_idx = children[u].iter().position(|&v| nodes[v].0 == right_max).unwrap();
        let mut hight_min = children[u].iter().map(|&v| nodes[v].1).min().unwrap();

        let first_relay = (left_min, hight_min);
        let mut vertical_group = children[u]
            .iter()
            .filter(|&&v| nodes[v].0 - left_min <= nodes[v].1 - hight_min)
            .collect::<Vec<&usize>>();
        vertical_group.sort_by(|&&a, &&b| nodes[a].1.cmp(&nodes[b].1));
        let mut horizontal_group = children[u]
            .iter()
            .filter(|&&v| nodes[v].1 - hight_min < nodes[v].0 - left_min)
            .collect::<Vec<&usize>>();
        horizontal_group.sort_by(|&&a, &&b| nodes[a].0.cmp(&nodes[b].0));

        ans_v.push((nodes[u].0, nodes[u].1, first_relay.0, first_relay.1));
        edges.push((nodes[u].0 as usize, nodes[u].1 as usize, first_relay.0 as usize, first_relay.1 as usize));

        {
            let mut cur = first_relay;
            for &&v in vertical_group.iter() {
                
                let (x, y) = nodes[v];
                let next = (left_min, y);
                ans_v.push((cur.0, cur.1, next.0, next.1));
                edges.push((cur.0 as usize, cur.1 as usize, next.0 as usize, next.1 as usize));
                ans_v.push((next.0, next.1, x, y));
                edges.push((next.0 as usize, next.1 as usize, x as usize, y as usize));
                cur = next;
                queue.push_back(v);
            }
        }

        {
            let mut cur = first_relay;
            for &&v in horizontal_group.iter() {
                let (x, y) = nodes[v];
                let next = (x, hight_min);
                ans_v.push((cur.0, cur.1, next.0, next.1));
                edges.push((cur.0 as usize, cur.1 as usize, next.0 as usize, next.1 as usize));
                ans_v.push((next.0, next.1, x, y));
                edges.push((next.0 as usize, next.1 as usize, x as usize, y as usize));
                cur = next;
                queue.push_back(v);
            }
        }
    }

    ans_v.retain(|(a, b, c, d)| !(a == c && b == d));

    Output {
        m: ans_v.len(),
        vec: ans_v,
    }
}

struct Input {
    n: usize,
    ab: Vec<(i32, i32)>,
}

impl Input {
    fn new() -> Input {
        input! {
            n: usize,
            ab: [(i32, i32); n],
        }

        Input { n, ab }
    }
}

struct Output {
    m: usize,
    vec: Vec<(i32, i32, i32, i32)>,
}

impl Output {
    fn new(m: usize, vec: Vec<(i32, i32, i32, i32)>) -> Output {
        Output { m, vec }
    }
}

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        // ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}
