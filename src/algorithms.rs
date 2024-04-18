pub fn my_merge_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    let mut v0 = Vec::with_capacity(v.len());

    if v.len() == 1 {
        v0.push(v[0]);
    } else {
        v0 = crate::merge_sorted_vectors(crate::sort_vector(&v[..(v.len()/2)]), crate::sort_vector(&v[(v.len()/2)..]));
    }
    *v = v0;

}

fn sort_vector<T: PartialOrd + Copy>(slice: &[T]) -> Vec<T> {

    let mut v0 = Vec::with_capacity(slice.len());

    if slice.len() == 1 {
        v0.push(slice[0]);
    } else {
        v0 = crate::merge_sorted_vectors(crate::sort_vector(&slice[..(slice.len()/2)]), crate::sort_vector(&slice[(slice.len()/2)..]));
    }
    v0
}
fn merge_sorted_vectors<T: PartialOrd + Copy>(v_left: Vec<T>, v_rigth: Vec<T>) -> Vec<T> {

    let mut v0 = Vec::with_capacity(v_left.len() + v_rigth.len());

    let mut pos_left = 0;
    let mut pos_rigth = 0;

    loop {
        if pos_left < v_left.len() && (pos_rigth == v_rigth.len() || v_left[pos_left] <= v_rigth[pos_rigth]) {
            v0.push(v_left[pos_left]);
            pos_left += 1;
        } else if pos_rigth < v_rigth.len() {
            v0.push(v_rigth[pos_rigth]);
            pos_rigth += 1;
        } else {
            break;
        }
    }

    v0
}
