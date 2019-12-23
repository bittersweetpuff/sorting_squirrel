///Function returns sorted array of values using selection sort
pub fn selection_sort<T: Ord>(array: &mut [T]){
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i..array.len() {
            if array[min_index] > array[j] {
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}