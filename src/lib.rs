#[macro_use]
macro_rules! answers {
    ( $( $problem_id:ident: $answer:expr ),* ) => {
        #[allow(dead_code)]
        pub struct Answers { $( pub $problem_id: i128, )* }
        #[allow(dead_code)]
        pub const ANSWERS: Answers = Answers { $( $problem_id: $answer, )* };
    };
}

#[macro_use]
macro_rules! solve {
    ( $problem_id:ident, $method:ident, $arg:expr, $solution:expr ) => {
        pub fn $method(n: i128) -> i128 {
            $solution(n)
        }

        paste::item! {
            #[test]
            fn [<$method _test>]() {
                assert_eq!(crate::answers::ANSWERS.$problem_id, $method( $arg ));
            }
        }
    };
}

mod answers;
pub mod solutions;
