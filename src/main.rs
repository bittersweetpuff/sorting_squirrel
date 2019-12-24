mod bubble_sort;
mod selection_sort;
mod insertion_sort;

pub use self::selection_sort::selection_sort;
pub use self::bubble_sort::bubble_sort;
pub use self::insertion_sort::insertion_sort;




///testing functions here
pub fn main(){
    let mut vec = vec![0, 21, 76, 123, 1, 23, 98, 565, -2, 652, 7];
    println!("presorted vector: {:?} ", vec);
    insertion_sort(&mut vec);
    println!("presorted vector: {:?} ", vec);
}