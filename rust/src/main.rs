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

// 有効かつ一番近い奴を親として木を作る
fn solve(input: &Input) -> Output {
    let n = input.n;
    let mut s = vec![(0, 0); 1];
    let mut nodes = vec![(0, 0); 1];
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

        let (x1, y1) = nodes[u];
        for &v in children[u].iter() {
            let (x2, y2) = nodes[v];
            ans_v.push((x1, y1, x2, y2));
            queue.push_back(v);
        }
    }

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
