use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.split_whitespace();
}

fn lower_bound(src: &[usize], elem: usize) -> Option<usize> {
    let mut ans = None;
    let mut left = 0;
    let mut right = src.len();
    let mut mid = (right - left) / 2 + left;
    let mut pmid = 0;

    while pmid != mid {
        pmid = mid;
        if src[mid] < elem {
            left = mid;
        } else if src[mid] >= elem {
            right = mid;
        }

        if src[left] == elem {
            ans = Some(left);
        }

        if right < src.len() && src[right] == elem {
            ans = Some(right);
        }
        mid = (right - left) / 2 + left;
    }
    ans
}

#[cfg(test)]
mod test_lower_bound {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn most_left() {
        let src = vec![0, 1, 2, 3];
        assert_eq!(lower_bound(&src, 0), Some(0));
    }

    #[test]
    fn middle() {
        let src = vec![0, 1, 2];
        assert_eq!(lower_bound(&src, 1), Some(1));
    }

    #[test]
    fn most_right() {
        let src = vec![0, 1, 2];
        assert_eq!(lower_bound(&src, 2), Some(2));
    }

    #[test]
    fn long_left() {
        let src = vec![0, 0, 0, 0, 1, 2];
        assert_eq!(lower_bound(&src, 0), Some(0));
    }

    #[test]
    fn long_middle() {
        let src = vec![0, 1, 1, 1, 1, 2];
        assert_eq!(lower_bound(&src, 1), Some(1));
    }

    #[test]
    fn long_right() {
        let src = vec![0, 1, 2, 2, 2, 2];
        assert_eq!(lower_bound(&src, 2), Some(2));
    }

    #[test]
    fn none_most_min() {
        let src = vec![1, 2, 3];
        assert_eq!(lower_bound(&src, 0), None);
    }

    #[test]
    fn none_middle() {
        let src = vec![1, 3];
        assert_eq!(lower_bound(&src, 2), None);
    }

    #[test]
    fn none_max() {
        let src = vec![1, 3];
        assert_eq!(lower_bound(&src, 4), None);
    }
}