use std::cmp::Ordering;

fn main(){
    let data = vec![1, 2, 5, 7, 8, 9, 10];
    let target = 8;
    let index = binary_search(&data, target);
    match index{
        Some(index) => {println!("Your target is {} in possition {}", target, index);}
        None => {println!("Target not found");}
    }
}

fn binary_search(data: &[i32], target: i32) -> Option<usize>{
    let mut down: usize = 0;
    let mut up: usize = data.len().checked_sub(1)?;

    while down <= up {
        let mean: usize = down + (up - down) / 2;
        match data[mean].cmp(&target){
            Ordering::Less => { down = mean + 1; }
            Ordering::Greater => { up = mean.checked_sub(1)?; }
            Ordering::Equal => { return Some(mean); }
        }
    }
    None
}
