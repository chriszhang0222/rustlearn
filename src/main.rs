
use std::collections::HashMap;
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
fn main() {
    println!("{}", longest("12", "34"));
}



// fn binarySearch(arr: &[i32], val: i32) -> i32{
//     let mut low:i32 = 0;
//     let mut high: i32 = (arr.len() - 1) as i32;
//     while low <= high{
//         let mut mid:i32 = low + (high-low)/2;
//         if arr[mid] == val {
//             return mid;
//         }else if arr[mid] < val{
//             low = mid + 1;
//         }else{
//             high = mid-1;
//         }
//     }
//     return low;
//
// }

