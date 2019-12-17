use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::collections::LinkedList;

fn main() {
    let f = std::fs::File::open("src.txt").expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut edges_count = 0u32;
    let mut edges: Vec<Vec<u32>> = vec![vec![0u32; 0]; 1000];
    let mut isolates = bit_set::BitSet::with_capacity(1000);
    for line in reader.lines() {
        if let Err(e) = line {
            panic!("Cannot read line {:?}", e)
        }
        let line = line.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        let edge1 = v.get(0).unwrap().parse::<u32>().unwrap();
        let edge2 = v.get(1).unwrap().parse::<u32>().unwrap();
        edges_count += 1;
        edges.get_mut(edge1 as usize).unwrap().push(edge2);
        edges.get_mut(edge2 as usize).unwrap().push(edge1);

        isolates.insert(edge1 as usize);
        isolates.insert(edge2 as usize);
    }

    print_answers(&mut edges_count, &mut edges, &mut isolates);
    
    let mut i = 0;
    let mut counter = 0;
    while i < 1000 {
        remove_references(i, &mut edges, &mut isolates);
        i += 17;
        counter += 1;
    }
    remove_references(225, &mut edges, &mut isolates);
    remove_references(227, &mut edges, &mut isolates);
    remove_references(141, &mut edges, &mut isolates);
    remove_references(559, &mut edges, &mut isolates);
    remove_references(881, &mut edges, &mut isolates);
    remove_references(570, &mut edges, &mut isolates);
    remove_references(59, &mut edges, &mut isolates);

    println!("{} vertices has been deleted", counter + 7);

    print_answers(&mut edges_count, &mut edges, &mut isolates);
}

fn remove_references(victim: u32, edges: &mut Vec<Vec<u32>>, isolates: &mut bit_set::BitSet) {
    let victim = victim as usize;
    edges.get_mut(victim).unwrap().clear();
    for i in 0..edges.len() {
        let x = edges.get_mut(i).unwrap();
        if !x.is_empty() {
            x.retain(|a| { *a as usize != victim });
            if x.is_empty() { isolates.remove(i); }
        }
    }
    isolates.insert(victim);
}

fn print_answers(edges_count: &mut u32, edges: &mut Vec<Vec<u32>>, isolates: &mut bit_set::BitSet<u32>)  {
    println!("There are {} edges in graph", edges_count);
    println!("There are {} isolates in graph", 1000 - isolates.len());
    println!("List of isolates:");
    for i in 0..1000 {
        if !isolates.contains(i) {
            println!("    {}", i)
        }
    }
    let mut max = 0u32;
    for i in 0..1000 {
        let tmp = edges.get(i).unwrap().len() as u32;
        if tmp > max { max = tmp }
    }
    println!("Maximal power of edge is {}", max);
    println!("The most powerful edges:");
    for i in 0..1000 {
        let tmp = edges.get(i).unwrap().len() as u32;
        if tmp == max {
            println!("    {}", i)
        }
    }
    let mut local_involvement = bit_set::BitSet::new();

    find_path_between(edges, &mut local_involvement, 66, 693);
    find_path_between(edges, &mut local_involvement, 839, 252);
    find_path_between(edges, &mut local_involvement, 330, 111);

    //search_and_analyze_component(&edges)
}

fn find_path_between(edges: &[Vec<u32>], involved_edges: &mut bit_set::BitSet, a: usize, b: usize) {
    let result = breadth_first_search(involved_edges, edges, a, b, true);

    match result {
        Ok(len) => println!("The shortest way between {} and {} is {}", a, b, len),
        Err(()) => println!("There is no way between {} and {} in this graph", a, b)
    }
    involved_edges.clear();
}

fn search_and_analyze_component(edges: &[Vec<u32>]) {
    let mut lead_vertex = 0;
    let mut max_vertices = 0;
    let mut involved = bit_set::BitSet::with_capacity(1000);
    for i in 0..1000 {
        if !involved.contains(i) {
            let before = involved.len();
            breadth_first_search(&mut involved, edges, i, 1000, false);
            let delta = involved.len() - before;
            if delta > max_vertices {
                lead_vertex = i;
                max_vertices = delta;
            }
        }
    }

    involved.clear();
    breadth_first_search(&mut involved, edges, lead_vertex, 1000, false);

    let mut max = 0usize;
    let vec = involved.into_bit_vec();
    for i in 0..1000 {
        if !vec[i] { continue }
        for j in 0..1000 {
            if i == j { continue }
            if !vec[j] { continue }

            let mut local_involvement = bit_set::BitSet::new();
            let path_length = breadth_first_search(&mut local_involvement, edges, i, j, false).unwrap();
            if path_length > max {
                max = path_length;
            }
        }
    }

    println!("Maximal diameter is {}", max);
}

type Joint = (usize, usize);
fn breadth_first_search(involved_edges: &mut bit_set::BitSet, edges: &[Vec<u32>], from: usize, to: usize, print_way: bool) -> Result<usize, ()> {
    let mut queue = std::collections::VecDeque::<usize>::new();
    let mut stack = std::collections::LinkedList::<Vec<Joint>>::new();
    let mut iteration = 1usize;
    queue.push_front(from);
    while !queue.is_empty() {
        let mut i = queue.len();
        let mut exploration_result = Vec::<Joint>::new();
        while i > 0 {
            let edge = queue.pop_back().unwrap();
            i -= 1;
            for child in edges.get(edge).unwrap() {
                let child = *child as usize;
                if involved_edges.contains(child) {
                    continue;
                }
                exploration_result.push((edge, child));
                if child == to {
                    if print_way {
                        stack.push_front(exploration_result);
                        analyze_way(&mut stack, to)
                    }
                    return Ok(iteration);
                }
                queue.push_front(child);
                involved_edges.insert(child);
            }
        }
        stack.push_front(exploration_result);
        iteration += 1
    }
    Err(())
}

fn analyze_way(way: &mut LinkedList<Vec<Joint>>, target: usize) {
    let mut stack = std::collections::LinkedList::<usize>::new();
    let mut current = target;
    stack.push_front(target);
    while !way.is_empty() {
        let mut step = way.pop_front().unwrap();
        for (from, to) in step {
            if to == current {
                current = from;
                break;
            }
        }
        stack.push_front(current);
    }

    while !stack.is_empty() {
        print!("{} ", stack.pop_front().unwrap());
    }
    println!()

}


