pub fn f1(tuple: &mut (u32, u32), flag: bool) -> &mut u32 {
    if !flag { &mut tuple.0 } else { &mut tuple.1 }
}

pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    if n >= slice.len() {
        panic!(
            "Index out of bounds: n ({}) must be less than slice length ({})",
            n,
            slice.len()
        );
    }
    &mut slice[slice.len() - n - 1]
}
pub fn f4(slice: &[u32]) -> [&[u32]; 4] {
    let len = slice.len();
    let chunk_size = len / 4;
    let remainder = len % 4;

    let sizes = [
        chunk_size + if remainder > 0 { 1 } else { 0 },
        chunk_size + if remainder > 1 { 1 } else { 0 },
        chunk_size + if remainder > 2 { 1 } else { 0 },
        chunk_size,
    ];

    let start_1 = 0;
    let start_2 = start_1 + sizes[0];
    let start_3 = start_2 + sizes[1];
    let start_4 = start_3 + sizes[2];

    [
        &slice[start_1..start_2],
        &slice[start_2..start_3],
        &slice[start_3..start_4],
        &slice[start_4..len],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let mut tuple = (42, 99);

        let ref_to_first = f1(&mut tuple, false);
        assert_eq!(*ref_to_first, 42);
        *ref_to_first = 100;
        assert_eq!(tuple.0, 100);

        let ref_to_second = f1(&mut tuple, true);
        assert_eq!(*ref_to_second, 99);
        *ref_to_second = 200;
        assert_eq!(tuple.1, 200);
    }

    #[test]
    fn test_f2() {
        let mut slice = [10, 20, 30, 40, 50];

        let first = f2(&mut slice, 0);
        assert_eq!(*first, 10);
        *first = 15;
        assert_eq!(slice[0], 15);

        let middle = f2(&mut slice, 2);
        assert_eq!(*middle, 30);
        *middle = 35;
        assert_eq!(slice[2], 35);

        let last = f2(&mut slice, 4);
        assert_eq!(*last, 50);
        *last = 55;
        assert_eq!(slice[4], 55);
    }

    #[test]
    fn test_f3() {
        let mut slice = [10, 20, 30, 40, 50];

        let last = f3(&mut slice, 0);
        assert_eq!(*last, 50);
        *last = 55;
        assert_eq!(slice[4], 55);

        let third_from_end = f3(&mut slice, 2);
        assert_eq!(*third_from_end, 30);
        *third_from_end = 35;
        assert_eq!(slice[2], 35);

        let first = f3(&mut slice, 4);
        assert_eq!(*first, 10);
        *first = 15;
        assert_eq!(slice[0], 15);
    }

    #[test]
    fn test_f4() {
        let slice_1 = [1, 2, 3, 4, 5, 6, 7, 8];
        let parts_1 = f4(&slice_1);
        assert_eq!(parts_1[0], &[1, 2]);
        assert_eq!(parts_1[1], &[3, 4]);
        assert_eq!(parts_1[2], &[5, 6]);
        assert_eq!(parts_1[3], &[7, 8]);

        let slice_2 = [1, 2, 3, 4, 5];
        let parts_2 = f4(&slice_2);
        assert_eq!(parts_2[0], &[1, 2]);
        assert_eq!(parts_2[1], &[3]);
        assert_eq!(parts_2[2], &[4]);
        assert_eq!(parts_2[3], &[5]);

        let slice_3 = [1, 2, 3, 4, 5, 6, 7];
        let parts_3 = f4(&slice_3);
        assert_eq!(parts_3[0], &[1, 2]);
        assert_eq!(parts_3[1], &[3, 4]);
        assert_eq!(parts_3[2], &[5, 6]);
        assert_eq!(parts_3[3], &[7]);
    }
}
