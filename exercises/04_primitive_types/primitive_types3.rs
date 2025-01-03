fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let temp="1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    let a = temp.to_owned()+temp.clone();

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
