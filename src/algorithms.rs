pub fn my_merge_sort<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    let mut v0 = Vec::with_capacity(v.len());

    if v.len() == 1 {
        return;
    } else {
        v0 = merge_sorted_vectors(sort_vector(&v[..(v.len()/2)]), sort_vector(&v[(v.len()/2)..]));
    }
    *v = v0;

}

fn sort_vector<T: PartialOrd + Copy>(slice: &[T]) -> Vec<T> {

    let mut v0 = Vec::with_capacity(slice.len());

    if slice.len() == 1 {
        v0.extend_from_slice(slice);
    } else {
        v0 = merge_sorted_vectors(sort_vector(&slice[..(slice.len()/2)]), sort_vector(&slice[(slice.len()/2)..]));
    }
    v0
}
fn merge_sorted_vectors<T: PartialOrd + Copy>(v_left: Vec<T>, v_rigth: Vec<T>) -> Vec<T> {

    let mut v0 = Vec::with_capacity(v_left.len() + v_rigth.len());

    let mut pos_left = 0;
    let mut pos_rigth = 0;

    loop {
        if pos_left < v_left.len() && pos_rigth < v_rigth.len() {
            if v_left[pos_left] <= v_rigth[pos_rigth] {
                v0.push(v_left[pos_left]);
                pos_left += 1;
            } else {
                v0.push(v_rigth[pos_rigth]);
                pos_rigth += 1;
            }
        } else if pos_left == v_left.len() {
            v0.extend_from_slice(&v_rigth[pos_rigth..]);
            break;
        } else {
            v0.extend_from_slice(&v_left[pos_left..]);
            break;
        }
    }

    v0
}

//------------

pub fn my_merge_sort_optmized<T: PartialOrd + Copy>(v: &mut Vec<T>) {

    if v.len() == 1 {
        return;
    }

    let mut v_left = v[..(v.len() / 2)].to_vec();
    let mut v_rigth = v[(v.len() / 2)..].to_vec();

    my_merge_sort_optmized(&mut v_left);
    my_merge_sort_optmized(&mut v_rigth);

    let mut pos_left = 0;
    let mut pos_rigth = 0;

    v.clear();

    loop {
        if pos_left < v_left.len() && pos_rigth < v_rigth.len() {
            if v_left[pos_left] <= v_rigth[pos_rigth] {
                v.push(v_left[pos_left]);
                pos_left += 1;
            } else {
                v.push(v_rigth[pos_rigth]);
                pos_rigth += 1;
            }
        } else if pos_left == v_left.len() {
            v.extend_from_slice(&v_rigth[pos_rigth..]);
            break;
        } else {
            v.extend_from_slice(&v_left[pos_left..]);
            break;
        }
    }
}