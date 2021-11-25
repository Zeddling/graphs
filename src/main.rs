extern crate arrayvec;
extern crate bit_vec;

use arrayvec::ArrayVec;
use bit_vec::BitVec;
use std::cell::RefCell;
use std::collections::LinkedList;

/**
 * Number of vertices
 */
const NUM_V: usize = 4;

/**
 * Returns true if a back edge is detected.
 * 
 * graph: reference to the graph;
 * position: the index of the node being visited;
 * visited: reference to the visited bit vector;
 * recStack: reference to the recursion stack of the traversal;
 */
fn helper<'a>(
    //  Use interior mutability pattern
    //  Know of a better solution?
    //  Previous
    graph: &'a RefCell<&mut ArrayVec<LinkedList<i32>, NUM_V>>, 
    position: usize, 
    visited: &mut BitVec, 
    rec_stack: &mut BitVec
) -> bool {
    visited.set(position, true);
    rec_stack.set(position, true);

    let vertices = &graph.borrow_mut()[position];

    for node in vertices.iter() {
        println!("Checking if vertice has a back edge");
        //  check whether vertice is in recursion stack
        if rec_stack.get(position) == Some(true) {
            return true;
        } else if *node == (position as i32) {
            return true;
        } else if visited.get(position) == None {
            if helper(&graph, *node as usize, visited, rec_stack) {
                return true;
            }
        }
    }

    rec_stack.set(position, false);
    false
}


/**
 * Wrapper function calls helper function on each vertice which has not been visited.
 */
fn is_cyclic(
    graph: &mut ArrayVec<LinkedList<i32>, NUM_V>, 
    num_vertices: usize
) -> bool {
    //  Track vertice visitations
    let mut visited = BitVec::from_elem(num_vertices, false);

    //  Tracks vertices in recursion stack of the traversal
    let mut rec_stack = BitVec::from_elem(num_vertices, false);

    for i in 0..num_vertices {
        if visited.get(i) == Some(false) {
            if helper(&RefCell::new(graph), i, &mut visited, &mut rec_stack) {
                return true
            }
        }
    }
    false
}


fn main() {
    let mut graph = ArrayVec::<LinkedList<i32>, NUM_V>::new();

    //  Initialize
    for _ in 0..NUM_V {
        graph.push(LinkedList::new());
    }
    graph[0].push_back(1);
    graph[0].push_back(2);
    graph[1].push_back(2);
    graph[2].push_back(0);
    graph[2].push_back(3);
    graph[3].push_back(3);

    let res = is_cyclic(&mut graph, NUM_V);
    println!("Is a cyclic graph: {}",  res);
}