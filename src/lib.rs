use std::{fmt::Debug, fmt::Display, marker::PhantomData, str::FromStr, time::Duration};

pub trait PuzzleInput
where
    Self: Sized,
{
    type Out;

    fn from_input(input: &str) -> Self::Out;
}

pub struct Blocks<T>(PhantomData<T>);

impl<T> PuzzleInput for Blocks<T>
where
    T: PuzzleInput,
{
    type Out = Vec<T::Out>;

    fn from_input(input: &str) -> Self::Out {
        input.split("\n\n").map(|l| T::from_input(l)).collect()
    }
}

pub struct Parsing<T>(PhantomData<T>);

impl<T> PuzzleInput for Parsing<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Out = Vec<T>;

    fn from_input(input: &str) -> Self::Out {
        input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| T::from_str(l).unwrap())
            .collect()
    }
}

pub struct As<T>(PhantomData<T>);

impl<T> PuzzleInput for As<T>
where
    T: From<String>,
{
    type Out = Vec<T>;

    fn from_input(input: &str) -> Self::Out {
        input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| T::from(String::from(l)))
            .collect()
    }
}

pub struct PuzzleSolution<T> {
    pub part1: T,
    pub part2: T,
    pub timings: Option<(Duration, Duration)>,
}

pub trait Solution {
    type Input: PuzzleInput;
    type Output;

    fn puzzle_input() -> &'static str;

    fn run(input: <Self::Input as PuzzleInput>::Out) -> (Self::Output, Self::Output) {
        let PuzzleSolution {
            part1,
            part2,
            timings: _,
        } = Self::timed_run(input);
        (part1, part2)
    }

    fn timed_run(input: <Self::Input as PuzzleInput>::Out) -> PuzzleSolution<Self::Output>;

    #[inline]
    fn parse_input(input: &str) -> <Self::Input as PuzzleInput>::Out {
        <Self::Input as PuzzleInput>::from_input(input)
    }

    #[inline]
    fn run_on(input: &str) -> (Self::Output, Self::Output) {
        let input = Self::parse_input(input);
        Self::run(input)
    }

    #[inline]
    fn run_on_input() -> (Self::Output, Self::Output) {
        let input = Self::puzzle_input();
        Self::run_on(input)
    }

    #[inline]
    fn timed_run_on_input() -> PuzzleSolution<Self::Output> {
        let input = Self::puzzle_input();
        let input = Self::parse_input(input);
        Self::timed_run(input)
    }

    #[inline]
    fn solve() -> PuzzleSolution<Box<dyn Display>>
    where
        Self::Output: Display + 'static,
    {
        let PuzzleSolution {
            part1,
            part2,
            timings,
        } = Self::timed_run_on_input();
        PuzzleSolution {
            part1: Box::new(part1),
            part2: Box::new(part2),
            timings,
        }
    }
}

#[macro_export]
macro_rules! register {

    ($file:literal; run($input:ident: $input_ty:ty) -> $output_ty:ty $runner:block) => {
        #[rustfmt::skip]
        register!($file; run($input: verbatim $crate::As<$input_ty>) -> $output_ty $runner);
    };

    ($file:literal; run($input:ident: parse $input_ty:ty) -> $output_ty:ty $runner:block) => {
        #[rustfmt::skip]
        register!($file; run($input: verbatim $crate::Parsing<$input_ty>) -> $output_ty $runner);
    };

    ($file:literal; run($input:ident: chunk $input_ty:ty) -> $output_ty:ty $runner:block) => {
        #[rustfmt::skip]
        register!($file; run($input: verbatim $crate::Blocks<$crate::As<$input_ty>>) -> $output_ty $runner);
    };

    ($file:literal; run($input:ident: verbatim $input_ty:ty) -> $output_ty:ty $runner:block) => {
        pub(crate) struct Solver;

        impl $crate::Solution for Solver {
            type Input = $input_ty;
            type Output = $output_ty;

            #[inline]
            fn puzzle_input() -> &'static str {
                ::std::include_str!($file)
            }

            #[inline]
            fn timed_run(mut $input: <$input_ty as $crate::PuzzleInput>::Out) -> $crate::PuzzleSolution<Self::Output> {
                let (part1, part2) = $runner;
                $crate::PuzzleSolution {
                    part1, part2, timings: None
                }
            }
        }
    };

    ($file:literal; ($input:ident: $input_ty:ty) -> $output_ty:ty { $part1:expr, $part2:expr }) => {
        #[rustfmt::skip]
        register!($file; ($input: verbatim $crate::As<$input_ty>) -> $output_ty { $part1, $part2 });
    };

    ($file:literal; ($input:ident: parse $input_ty:ty) -> $output_ty:ty { $part1:expr, $part2:expr}) => {
        #[rustfmt::skip]
        register!($file; ($input: verbatim $crate::Parsing<$input_ty>) -> $output_ty { $part1, $part2 });
    };

    ($file:literal; ($input:ident: chunk $input_ty:ty) -> $output_ty:ty { $part1:expr, $part2:expr}) => {
        #[rustfmt::skip]
        register!($file; ($input: verbatim $crate::Blocks<$crate::As<$input_ty>>) -> $output_ty { $part1, $part2 });
    };

    ($file:literal; ($input:ident: verbatim $input_ty:ty) -> $output_ty:ty { $part1:expr, $part2:expr }) => {
        pub(crate) struct Solver;

        impl $crate::Solution for Solver {
            type Input = $input_ty;
            type Output = $output_ty;

            #[inline]
            fn puzzle_input() -> &'static str {
                ::std::include_str!($file)
            }

            #[inline]
            fn timed_run(mut $input: <$input_ty as $crate::PuzzleInput>::Out) -> $crate::PuzzleSolution<Self::Output> {
                let start = ::std::time::Instant::now();
                let part1 = $part1;
                let part1_time = start.elapsed();
                let start = ::std::time::Instant::now();
                let part2 = $part2;
                let part2_time = start.elapsed();

                $crate::PuzzleSolution {
                    part1, part2, timings: Some((part1_time, part2_time))
                }
            }
        }
    };
}
