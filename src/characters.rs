use colored::{self, ColoredString, Colorize};

//type Character{
//    fn display() -> String;
//};
//
//struct Character {
//    name: String,
//    color: String,
//}

pub fn ferris() -> ColoredString {
    r#"
            _~^~^~_
        \) / o o \ (/
          '_ - _'
          / '-----' \
"#
    .to_string()
    .bright_red()
}

pub fn cow() -> ColoredString {
    r#"
        ^__^
        (oo)\_______
        (__)\       )\/\
            ||----w |
            ||     ||
    "#
    .to_string()
    .yellow()
}

pub fn dragon() -> ColoredString {
    r#"               / \  //\
               |\___/|      /   \//  \\
               /0  0  \__  /    //  | \ \    
              /     /  \/_/    //   |  \  \  
              @_^_@'/   \/_   //    |   \   \ 
              //_^_/     \/_ //     |    \    \
           ( //) |        \///      |     \     \
         ( / /) _|_ /   )  //       |      \     _\
       ( // /) '/,_ _ _/  ( ; -.    |    _ _\.-~        .-~~~^-.
     (( / / )) ,-{        _      `-.|.-~-.           .~         `.
    (( // / ))  '/\      /                 ~-. _ .-~      .-~^-.  \
    (( /// ))      `.   {            }                   /      \  \
     (( / ))     .----~-.\        \-'                 .~         \  `. \^-.
                 ///.----..>        \             _ -~             `.  ^-`  ^-_
                   ///-._ _ _ _ _ _ _}^ - - - - ~                     ~-- ,.-~
"#
    .to_string()
    .red()
}

pub fn bunny() -> ColoredString {
    r#"
        (\(\ 
        ( -.-) 
        o_(")(")
    "#
    .to_string()
    .bright_white()
}
