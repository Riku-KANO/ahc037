use std::collections::{BTreeMap, VecDeque};

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

fn solve(input: &Input) -> Output {
    let n = input.n;
    let mut nodes = vec![(0, 0); 1];
    let mut edges = Vec::<(usize, usize, usize, usize)>::new();
    let mut parents = vec![0usize; input.n + 1];
    let mut children: Vec<Vec<usize>> = vec![vec![]; input.n + 1];

    let mut parents2 = BTreeMap::<(i32, i32), (i32, i32)>::new();
    let mut children2 = BTreeMap::<(i32, i32), Vec<(i32, i32)>>::new();

    parents2.insert((0, 0), (0, 0));

    for ab in input.ab.iter() {
        nodes.push(*ab);
    }

    for (i, &(x1, y1)) in nodes.iter().enumerate() {
        // children
        if x1 == 0 && y1 == 0 {
            continue;
        }

        let mut dist_min = std::i32::MAX;
        for (j, &(x2, y2)) in nodes.iter().enumerate() {
            // parent candidate
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

    let mut visited = vec![false; n + 1];
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
            parents2.insert((x2, y2), (x1, y1));
            children2.entry((x1, y1)).or_default().push((x2, y2));
            continue;
        }

        // 子が２個以上の場合
        let left_min = children[u].iter().map(|&v| nodes[v].0).min().unwrap();
        let hight_min = children[u].iter().map(|&v| nodes[v].1).min().unwrap();

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
        edges.push((
            nodes[u].0 as usize,
            nodes[u].1 as usize,
            first_relay.0 as usize,
            first_relay.1 as usize,
        ));
        parents2.insert(first_relay, nodes[u]);
        children2.entry(nodes[u]).or_default().push(first_relay);

        {
            let mut cur = first_relay;
            for &&v in vertical_group.iter() {
                let (x, y) = nodes[v];
                let next = (left_min, y);
                ans_v.push((cur.0, cur.1, next.0, next.1));
                edges.push((
                    cur.0 as usize,
                    cur.1 as usize,
                    next.0 as usize,
                    next.1 as usize,
                ));
                parents2.insert((next.0, next.1), (cur.0, cur.1));
                children2
                    .entry((cur.0, cur.1))
                    .or_default()
                    .push((next.0, next.1));

                ans_v.push((next.0, next.1, x, y));
                edges.push((next.0 as usize, next.1 as usize, x as usize, y as usize));
                parents2.insert((x, y), (next.0, next.1));
                children2.entry((next.0, next.1)).or_default().push((x, y));

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
                edges.push((
                    cur.0 as usize,
                    cur.1 as usize,
                    next.0 as usize,
                    next.1 as usize,
                ));
                parents2.insert((next.0, next.1), (cur.0, cur.1));
                children2
                    .entry((cur.0, cur.1))
                    .or_default()
                    .push((next.0, next.1));

                ans_v.push((next.0, next.1, x, y));
                edges.push((next.0 as usize, next.1 as usize, x as usize, y as usize));
                parents2.insert((x, y), (next.0, next.1));
                children2.entry((next.0, next.1)).or_default().push((x, y));

                cur = next;
                queue.push_back(v);
            }
        }
    }

    while get_time() < 1.8 {
        let mut queue2 = VecDeque::<(i32, i32)>::new();
        for &(x, y) in children2.get(&(0, 0)).unwrap() {
            queue2.push_back((x, y));
        }

        let mut v = vec![];
        v.push((0, 0));
        queue2.push_back((0, 0));
        let mut visited2 = BTreeMap::<(i32, i32), bool>::new();
        while let Some((x, y)) = queue2.pop_front() {
            if visited2.get(&(x, y)).is_some() {
                continue;
            }
            visited2.insert((x, y), true);
            if let Some(children) = children2.get(&(x, y)) {
                for &(cx, cy) in children.iter() {
                    v.push((cx, cy));
                    queue2.push_back((cx, cy));
                }
            }
        }

        for (i, &(tx, ty)) in v.iter().enumerate() {
            if (tx, ty) == (0, 0) {
                continue;
            }

            let p = *parents2.get(&(tx, ty)).unwrap();
            let mut midpoint = (0, 0);
            let cost = (tx - p.0).abs() + (ty - p.1).abs();
            let mut max_diff = 0;
            let mut target_pc_pair = ((0, 0), (0, 0));

            for (&(px, py), children) in children2.iter() {
                if px > tx || py > ty {
                    continue;
                }

                for &(cx, cy) in children.iter() {
                    let cur_cost = cost;
                    // left side
                    if px <= tx && cx <= tx && py <= ty && cy >= ty {
                        let (mx, my) = (cx, ty);
                        let next_cost = (cx-tx).abs();
                        let diff = cur_cost - next_cost;
                        if diff > max_diff {
                            max_diff = diff;
                            target_pc_pair = ((px, py), (cx, cy));
                            midpoint = (mx, my);
                        }
                    }

                    // lower side
                    if py <= ty && cy <= ty && px <= tx && cx >= tx {
                        let (mx, my) = (tx, cy);
                        let next_cost = (ty-my).abs();

                        let diff = cur_cost - next_cost;
                        if diff > max_diff {
                            max_diff = diff;
                            target_pc_pair = ((px, py), (cx, cy));
                            midpoint = (mx, my);
                        }
                    }
                }
            }

            if max_diff > 0 {
                let (px, py) = target_pc_pair.0;
                let (cx, cy) = target_pc_pair.1;
                children2
                    .entry((px, py))
                    .or_default()
                    .retain(|&(x, y)| !(x == cx && y == cy));
                children2
                    .entry(p)
                    .or_default()
                    .retain(|&(x, y)| !(x == tx && y == ty));

                parents2.insert((tx, ty), midpoint);
                parents2.insert(midpoint, (px, py));
                parents2.insert((cx, cy), midpoint);

                children2.entry(midpoint).or_default().push((tx, ty));
                children2.entry(midpoint).or_default().push((cx, cy));

                children2.entry((px, py)).or_default().push(midpoint);
            }
        }
    }

    let mut ans_v = vec![];
    let mut queue3 = VecDeque::<(i32, i32)>::new();
    queue3.push_back((0, 0));
    let mut visited3 = BTreeMap::<(i32, i32), bool>::new();
    while let Some((x, y)) = queue3.pop_front() {
        if visited3.get(&(x, y)).is_some() {
            continue;
        }
        visited3.insert((x, y), true);
        if let Some(children) = children2.get(&(x, y)) {
            for &(cx, cy) in children.iter() {
                ans_v.push((x, y, cx, cy));
                queue3.push_back((cx, cy));
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
