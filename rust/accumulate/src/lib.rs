pub fn map_function<F>(input: &mut [i32], f: &F) -> Vec<i32>
    where F: Fn(i32) -> i32
{
    let mut result = Vec::with_capacity(input.len());

    for arg in input {
        result.push(f(*arg));
    }

    result
}

pub fn map_closure<F>(input: &mut [i32], f: F) -> Vec<i32>
    where F: Fn(i32) -> i32
{
    map_function(input, &f)
}
