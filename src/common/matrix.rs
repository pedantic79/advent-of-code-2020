pub fn rotate_right<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();
    for i in 0..(len / 2) {
        for j in i..(len - i - 1) {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[len - 1 - j].as_mut()[i];
            a[len - 1 - j].as_mut()[i] = a[len - 1 - i].as_mut()[len - 1 - j];
            a[len - 1 - i].as_mut()[len - 1 - j] = a[j].as_mut()[len - 1 - i];
            a[j].as_mut()[len - 1 - i] = temp;
        }
    }
}

pub fn rotate_left<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();
    for i in 0..(len / 2) {
        for j in i..(len - i - 1) {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[j].as_mut()[len - 1 - i];
            a[j].as_mut()[len - 1 - i] = a[len - 1 - i].as_mut()[len - 1 - j];
            a[len - 1 - i].as_mut()[len - 1 - j] = a[len - 1 - j].as_mut()[i];
            a[len - 1 - j].as_mut()[i] = temp;
        }
    }
}

pub fn rotate_bottom<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();

    if len % 2 == 1 {
        for j in 0..(len / 2) {
            a[len / 2].as_mut().swap(j, len - j - 1);
        }
    }

    for i in 0..(len / 2) {
        for j in 0..len {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[len - i - 1].as_mut()[len - j - 1];
            a[len - i - 1].as_mut()[len - j - 1] = temp;
        }
    }
}

pub fn flip<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    for row in a.iter_mut() {
        row.as_mut().reverse();
    }
}

pub fn rotate_right_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated: Vec<Vec<T>> = vec![vec![T::default(); rows]; cols];
    for i in 0..rows {
        for (j, &v) in a[rows - 1 - i].as_ref().iter().enumerate() {
            rotated[j][i] = v;
        }
    }

    rotated
}

pub fn rotate_left_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated = vec![vec![T::default(); rows]; cols];
    for (i, row) in a.iter().enumerate() {
        for j in 0..cols {
            rotated[cols - 1 - j][i] = row.as_ref()[j];
        }
    }

    rotated
}

pub fn rotate_bottom_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated = vec![vec![T::default(); cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            rotated[rows - 1 - i][cols - 1 - j] = a[i].as_ref()[j];
        }
    }

    rotated
}
