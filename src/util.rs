pub trait SliceExt: private::SliceExtSealed {
    type Item;

    fn split_once_<F>(&self, pred: F) -> Option<(&Self, &Self)>
    where
        F: FnMut(&Self::Item) -> bool;
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn split_once_<F>(&self, pred: F) -> Option<(&Self, &Self)>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        let i = self.iter().position(pred)?;

        Some((&self[..i], &self[i + 1..]))
    }
}

pub trait U8SliceExt: private::U8SliceExtSealed {
    fn split_first_i8(&self) -> Option<(i8, &Self)>;
    fn split_first_u8(&self) -> Option<(u8, &Self)>;
    fn split_first_u16le(&self) -> Option<(u16, &Self)>;
}

impl U8SliceExt for [u8] {
    fn split_first_i8(&self) -> Option<(i8, &Self)> {
        let (&value, remain) = self.split_first()?;
        let value = value as i8;

        Some((value, remain))
    }

    fn split_first_u8(&self) -> Option<(u8, &Self)> {
        let (&value, remain) = self.split_first()?;

        Some((value, remain))
    }

    fn split_first_u16le(&self) -> Option<(u16, &Self)> {
        let (&value, remain) = self.split_first_chunk::<2>()?;
        let value = u16::from_le_bytes(value);

        Some((value, remain))
    }
}

mod private {
    pub trait SliceExtSealed {}
    impl<T> SliceExtSealed for [T] {}

    pub trait U8SliceExtSealed {}
    impl U8SliceExtSealed for [u8] {}
}
