const ARR_SIZE : usize = 10;

fn main() {
    println!("Basic Implementation of Bubble Sort");

    let mut arr : [i32; ARR_SIZE] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    print!("Before bubble sort -> ");
    println!("{:?}", arr);

    bubble_sort(&mut arr);

    print!("After bubble sort -> ");
    println!("{:?}", arr);

    
}

//Sort values in ascending order
fn bubble_sort(arr : &mut [i32; ARR_SIZE]){

    for i in 0..ARR_SIZE {
        for j in (i+1)..ARR_SIZE {
            
            if arr[j] < arr[i]{
                let temp = arr[j];
                arr[j] = arr[i];
                arr[i] = temp;
            } 
        }
    }
}