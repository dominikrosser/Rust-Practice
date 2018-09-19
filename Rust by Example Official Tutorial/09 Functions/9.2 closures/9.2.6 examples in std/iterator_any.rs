fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // 'iter()' for vecs yields '&i32'. Destructure to 'i32'
    println!("2 in vec1: {}", v1.iter().any(|&x| x == 2));
    // 'into_iter()' for vecs yields 'i32'. No destructuring required
    println!("2 in vec2: {}", v2.into_iter().any(|x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    // 'iter()' for arrays yiehds '&i32'
    println!("2 in array1: {}", arr1.iter().any(|&x| x == 2));
    // 'into_iter()' for arrays unusually yields '&i32'
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
