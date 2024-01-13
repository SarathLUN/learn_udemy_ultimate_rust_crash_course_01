use core::str;
use std::{collections::{HashMap, VecDeque, LinkedList, HashSet, BinaryHeap, BTreeMap, BTreeSet}, i32};

fn main() {
    // Vector
    let mut vector: Vec<i32> = vec![1,2,3];
    vector.push(4);
    println!("my vector after pushed: {:?}", vector);
    vector.pop();
    println!("my vector after popped: {:?}", vector);

    // HashMap
    let mut hast_map: HashMap<u8, bool> = HashMap::new();
    hast_map.insert(42, true);
    println!("my hast_map: {:?}", hast_map);
    hast_map.remove(&42);
    println!("my hast_map: {:?}", hast_map);

    // other collections
    let mut vec_deque: VecDeque<i32> = VecDeque::new();
    vec_deque.push_front(3);
    println!("my vec_deque: {:?}", vec_deque);
    vec_deque.push_back(7);
    println!("my vec_deque after push_back: {:?}",vec_deque);
    vec_deque.push_front(8);
    println!("my vec_deque after push_front: {:?}",vec_deque);

    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.push_back(6);
    println!("linked_list: {:?}", linked_list);

    let mut hash_set: HashSet<i32> = HashSet::new();
    hash_set.insert(4);
    println!("hash_set: {:?}", hash_set);

    let bi_heap: BinaryHeap<i32> = BinaryHeap::from(vec![3, 1, 4, 1, 5, 9]);
    println!("bi_heap: {:?}",bi_heap);

    let mut b_tree_map: BTreeMap<i32, &str> = BTreeMap::new();
    b_tree_map.insert(3, "three");
    println!("b_tree_map: {:?}", b_tree_map);

    let mut b_tree_set: BTreeSet<i32> = BTreeSet::new();
    b_tree_set.insert(8);
    println!("b_tree_set: {:?}", b_tree_set);

}
