#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let numbers = [0.7f32, -0.4, 123.0];
        let answers = [Q7(89), Q7(-51), Q7(127)];

        for (number, answer) in numbers.into_iter().zip(answers) {
            assert_eq!(Q7::from(number), answer)
        }
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
