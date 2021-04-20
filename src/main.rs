mod second;
mod lib;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
use std::ptr::null;
use rustlearn::{test_in_lib, Node};

pub struct Point<T>{
    x:T,
    y:T
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans = vec![-1,-1];
    let mut map:HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    for num in &nums{
        if map.contains_key(&(target-num)) {
            ans[0] = *map.get(&(target - num)).unwrap();
            ans[1] = count;
            break;
        }else{
            map.insert(*num, count);
        }
    }
    return ans;
}


pub trait Summary{
    fn Summarize(&self) -> String;
}

pub trait Display{
    fn Display(&self) -> String;
}
pub fn notify(item: impl Summary + Display){
    item.Summarize();
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    return largest;
}

fn longest<'a>(s1: &'a str, s2:&'a str) -> String{
    if s1.len() > s2.len(){
        return String::from("123");
    }else{
        return String::from("1456");
    }
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    for i in 0..nums.len(){
        if i>0 && nums[i] == nums[i-1]{
            continue;
        }
        let mut j = i+1;
        let mut k = nums.len()-1;
        while j < k{
            let target = nums[i] + nums[j] + nums[k];
            if target == 0{
                ans.push(vec![nums[i], nums[j], nums[k]]);
                while j < k && nums[j] == nums[j+1]{
                    j += 1;
                }
                while j < k && nums[k] == nums[k-1]{
                    k -= 1;
                }
                j += 1;
                k -= 1;
            }else if target > 0{
                k -= 1;
            }else{
                j += 1;
            }
        }

    }
    return ans;

}

fn changeString(s1: String){
    let ans = three_sum(vec![-1,0,1]);

}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans:Vec<Vec<i32>> = Vec::new();
    if candidates.len() == 0{
        return ans;
    }
    let mut candidates = candidates;

    let mut tmp:Vec<i32> = Vec::new();
    dfs(&mut ans, &candidates, &mut tmp, 0, 0);
    return ans;
}

pub fn dfs(ans: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, tmp: &mut Vec<i32>, target: i32, index: usize){
    if target == 0{
        ans.push(tmp.to_owned());
        return;
    }

    for i in index..candidates.len(){
        if candidates[i] > target{
            break;
        }
        tmp.push(candidates[i]);
        dfs(ans, candidates, tmp, target-candidates[i], i+1);
        tmp.remove(tmp.len()-1);
    }
    return;
}



fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    return s.len();
}


fn add_element(num: &mut Vec<i32>){
    num.push(1);
}

// pub fn length_of_longest_substring(s: String) -> i32 {
//     let mut set = HashSet::new();
//     let start = 0;
//     let end = 0;
//     let max_ = 0;
//     let chars = s.bytes();
//
// }


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    let mut cp = list;

    for &item in list.iter() {
        let c = item;
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn some_fun<T, U>(t: T, u: U) -> String where T: Display + Clone, U: Clone{
    String::from("ok")
}

fn longest2<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}

pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

struct MyBox<T>(T);

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    // while let Some(mut boxed_node) = curr {
    //     curr.unwrap().next
    //     let mut next = boxed_node.next.take();
    //     boxed_node.next = prev.take();
    //     prev = Some(boxed_node);
    //     curr = next.take();
    // }
    // curr.is_some()

    while curr.is_some(){
        let mut next = curr.unwrap().next;
        curr.unwrap().next = prev;
        prev = curr;
        curr = next;
    }

    prev
}

fn main() {
    let mut head = ListNode::new(1);
    head.next = Option::from(Box::new(ListNode::new(2)));
    let next_node = head.next;
    let next_node2 = head.next.take();
    let t = match next_node {
        Some(Box) => 1,
        None => 2,
    };



}

fn binarySearch(arr: &[i32], val: i32) -> usize{
    let mut low:usize = 0;
    let mut high = (arr.len()-1);
    while low <= high{
        let mut mid = low + (high-low/2);
        if arr[mid] == val{
            return mid;
        }else if arr[mid] < val{
            low = mid+1;
        }else{
            high = mid-1;
        }
    }
    return low
}


