pub fn my_merge_sort_it<T: PartialOrd + Copy>(v: &mut Vec<T>) -> usize {

    let mut v0 = Vec::with_capacity(v.len());

    let mut count = 0;

    if v.len() == 1 {
        v0.push(v[0]);
        count = 1;
    } else {
        let left_vec = sort_vector(&v[..(v.len()/2)]);
        let rigth_vec = sort_vector(&v[(v.len()/2)..]);
        count = left_vec.1 + rigth_vec.1;
        v0 = merge_sorted_vectors(left_vec.0, rigth_vec.0, &mut count);

    }
    *v = v0;

    count

}

fn sort_vector<T: PartialOrd + Copy>(slice: &[T]) -> (Vec<T>, usize) {

    let mut v0 = Vec::with_capacity(slice.len());

    let mut count= 0;

    if slice.len() == 1 {
        v0.push(slice[0]);
        count += 1;
    } else {
        let left_vec = sort_vector(&slice[..(slice.len()/2)]);
        let rigth_vec = sort_vector(&slice[(slice.len()/2)..]);
        count = left_vec.1 + rigth_vec.1;
        v0 = merge_sorted_vectors(left_vec.0, rigth_vec.0, &mut count);
    }
    (v0, count)
}
fn merge_sorted_vectors<T: PartialOrd + Copy>(v_left: Vec<T>, v_rigth: Vec<T>, count: &mut usize) -> Vec<T> {

    let mut v0 = Vec::with_capacity(v_left.len() + v_rigth.len());

    let mut pos_left = 0;
    let mut pos_rigth = 0;

    loop {
        if pos_left < v_left.len() && pos_rigth < v_rigth.len() {
            if v_left[pos_left] <= v_rigth[pos_rigth] {
                v0.push(v_left[pos_left]);
                pos_left += 1;
                *count += 1;
            } else {
                v0.push(v_rigth[pos_rigth]);
                pos_rigth += 1;
                *count += 1;
            }
        } else if pos_left == v_left.len() {
            v0.extend_from_slice(&v_rigth[pos_rigth..]);
            *count += 1;
            break;
        } else {
            v0.extend_from_slice(&v_left[pos_left..]);
            *count += 1;
            break;
        }
    }

    v0
}
