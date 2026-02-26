pub enum Either<A, B> {
    A(A),
    B(B),
    Neither
}

impl<A, B> Either<A, B> {
    pub fn is_a(&self) -> bool {
        return match self {
            Self::A(_) => true,
            _ => false
        };
    }

    pub fn is_b(&self) -> bool {
        return match self {
            Self::B(_) => true,
            _ => false
        };
    }

    pub fn is_neither(&self) -> bool {
        return match self {
            Self::Neither => true,
            _ => false
        };
    }

    pub fn get_a(self) -> Option<A> {
        return match self {
            Self::A(a) => Option::Some(a),
            _ => Option::None
        };
    }

    pub fn get_b(self) -> Option<B> {
        return match self {
            Self::B(b) => Option::Some(b),
            _ => Option::None
        };
    }
}

impl<A: Clone, B: Clone> Clone for Either<A, B> {
    fn clone(&self) -> Self {
        return match self {
            Self::A(a) => Self::A(a.clone()),
            Self::B(b) => Self::B(b.clone()),
            Self::Neither => Self::Neither
        };
    }
}

impl<A: Copy, B: Copy> Copy for Either<A, B> {}