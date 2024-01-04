pub fn rotate_right<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
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
    for row in a.iter_mut() {
        row.as_mut().reverse();
    }
}
