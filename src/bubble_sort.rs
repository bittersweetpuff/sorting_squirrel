///Function returns sorted array of values using bubble sort
pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}