//https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
pub fn next_perm(v : &mut Vec<u8>)-> bool {
    let vl = v.len();   
    let mut i = vl-2;
    let mk;
    loop{
        if v[i] < v[i+1] {
            mk = i;
            break;
        }
        if i == 0 {
            return false;
        }
        i-=1;
    }
    let mut ml = 0;
    i = vl-1;
    while i > mk {
        if v[mk] < v[i] {
            ml = i;
            break;
        }
        i-=1;
    }
    let mut t = v[mk];
    v[mk] = v[ml];
    v[ml] = t;
    
    i = mk + 1;
    let mut j = vl-1;
    while i < j {
        t = v[i];
        v[i] = v[j];
        v[j] = t;
        i+=1;
        j-=1;
    }
    true
}