pub fn extended_euclid(x: i128, y: i128) -> (i128, (i128, i128)) {
    let (mut d, mut d_prev) = (x, y);
    let (mut a, mut a_prev) = (1, 0);
    let (mut b, mut b_prev) = (0, 1);

    let mut s: i128;
    let mut t: i128;
    while d_prev != 0 {
        let q = d / d_prev;

        s = d;
        t = d_prev;

        d = t;
        d_prev = s - q * t;

        s = a;
        t = a_prev;

        a = t;
        a_prev = s - q * t;

        s = b;
        t = b_prev;

        b = t;
        b_prev = s - q * t;
    }

    (d, (a, b))
}

pub fn chinese_remainder(linear_congruences: &[(i128, i128)]) -> Option<i128> {
    // Solves a system of linear congruences in `x` given the form of pairs of (`a_i`, `n_i`) such that:
    // `x` === `a_1` (mod `n_1`)
    // `x` === `a_2` (mod `n_2`)
    // ...
    // `x` === `a_k` (mod `n_k`)
    // (where the input has length k)
    //
    // Outputs a solution (if one exists) `a` such that:
    // `a` is a solution to the given congruences
    // `a` is unique modulo `n_1` * `n_2` * ... * `n_k`
    let mut linear_congruences = linear_congruences.iter();
    if let Some((a_0, n_0)) = linear_congruences.next() {
        let mut a = *a_0;
        let mut n = *n_0;
        for (a_i, n_i) in linear_congruences {
            let (d_i, (u_i, v_i)) = extended_euclid(n, *n_i);
            if d_i != 1 {
                return None;
            }
            a = a * v_i * *n_i + *a_i * u_i * n;
            n *= n_i;

            // Normalise x to 0 <= a < n.  The two modulus operations account for both +ive and -ive values of a.
            a = ((a % n) + n) % n;
        }

        return Some(a);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclid() {
        let (d, (a, b)) = extended_euclid(19, 25);
        assert_eq!(a * 19 + b * 25, d);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_crt() {
        let x = chinese_remainder(&vec![(0, 3), (3, 4), (4, 5)]).unwrap();

        assert_eq!(0, x % 3);
        assert_eq!(3, x % 4);
        assert_eq!(4, x % 5);

        assert_eq!(x % 60, x);
    }
}
