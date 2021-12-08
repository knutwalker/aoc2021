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

pub trait Solution {
    type Input: PuzzleInput;
    type Output;

    fn puzzle_input() -> &'static str;

    fn run(input: <Self::Input as PuzzleInput>::Out) -> (Self::Output, Self::Output) {
        Self::timed_run(input).0
    }

    fn timed_run(
        input: <Self::Input as PuzzleInput>::Out,
    ) -> ((Self::Output, Self::Output), Option<(Duration, Duration)>);

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
    fn timed_run_on_input() -> ((Self::Output, Self::Output), Option<(Duration, Duration)>) {
        let input = Self::puzzle_input();
        let input = Self::parse_input(input);
        Self::timed_run(input)
    }

    #[inline]
    fn solve() -> (
        Box<dyn Display>,
        Box<dyn Display>,
        Option<(Duration, Duration)>,
    )
    where
        Self::Output: Display + 'static,
    {
        let ((res1, res2), times) = Self::timed_run_on_input();
        (Box::new(res1), Box::new(res2), times)
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
            fn timed_run(mut $input: <$input_ty as $crate::PuzzleInput>::Out) -> ((Self::Output, Self::Output), ::std::option::Option<(::std::time::Duration, ::std::time::Duration)>) {
                ($runner, None)
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
            fn timed_run(mut $input: <$input_ty as $crate::PuzzleInput>::Out) -> ((Self::Output, Self::Output), ::std::option::Option<(::std::time::Duration, ::std::time::Duration)>) {
                let start = ::std::time::Instant::now();
                let part1 = $part1;
                let part1_time = start.elapsed();
                let start = ::std::time::Instant::now();
                let part2 = $part2;
                let part2_time = start.elapsed();

                ((part1, part2), Some((part1_time, part2_time)))
            }
        }
    };
}
