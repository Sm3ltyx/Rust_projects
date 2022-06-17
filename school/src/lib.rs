mod class;

pub use crate::class::teacher;

pub fn interact_with_teacher() {
    teacher::hand_over_homework();
    teacher::ask_for_grades();
    teacher::ask_for_help();
    teacher::hand_over_homework();
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝