fn main() {
    let vec_1 = vec![3, 2, 1];
    let vec_2 = vec![1, 2, 3];

    let answer = foo(&vec_1, &vec_2);

    print!(
        "asnwer = {}\nvec_1 = {:?}\nvec_2 = {:?}",
        answer, vec_1, vec_2
    );
}

fn foo(vec_1: &Vec<i32>, vec_2: &Vec<i32>) -> i32 {
    vec_1[0] + vec_2[0]
}
