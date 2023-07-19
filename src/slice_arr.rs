pub fn run() {
    let num = vec![1, 2, 3, 4, 5, 6, 7];
    let slices = num.chunks(3).collect::<Vec<_>>();

    let index = 0;

    let slice = slices[index];
    let len = slice.len();
    dbg!(slices);
    dbg!(slice);
    dbg!(len);
}
