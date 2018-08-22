/// What should the type of _function be?
pub fn map_function(input: Vec<i32>, _function: &Fn(i32) -> i32) -> Vec<i32> {
    map_closure(input, _function)
}

/// What should the type of _closure be?
pub fn map_closure<F>(input: Vec<i32>, _closure: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for i in input {
        result.push(_closure(i));
    }
    result
}
