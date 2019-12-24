///Function returns sorted array of values using insertion sort
pub fn insertion_sort<T: Ord>(array: &mut [T]){
        for i in 0..array.len() {
                for j in (1..i + 1).rev() {
                        if array[j] < array[j-1] {
                                array.swap(j, j-1);
                        }
                        else {
                                break;
                        }
                }
        }
}