// auto-generated: "lalrpop 0.19.5"
// sha3: 40a05932eaa4677c482b06bee77aec59fc091e727b436e1a8c7531427ec4fa
use crate::LexError;
use crate::lexer;
use crate::codegen::ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__LokFile {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::LexError;
    use crate::lexer;
    use crate::codegen::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(Vec<u8>),
        Variant2(String),
        Variant3(core::option::Option<lexer::Token>),
        Variant4(ast::Type),
        Variant5(core::option::Option<ast::Type>),
        Variant6((core::option::Option<String>, ast::Type)),
        Variant7(alloc::vec::Vec<(core::option::Option<String>, ast::Type)>),
        Variant8(ast::Expression),
        Variant9(alloc::vec::Vec<ast::Expression>),
        Variant10(alloc::vec::Vec<String>),
        Variant11(core::option::Option<(core::option::Option<String>, ast::Type)>),
        Variant12(ast::Block),
        Variant13(Vec<(core::option::Option<String>, ast::Type)>),
        Variant14(Vec<ast::Expression>),
        Variant15(core::option::Option<ast::Expression>),
        Variant16(core::option::Option<String>),
        Variant17(Vec<ast::TopLevelDecl>),
        Variant18(ast::NSIdent),
        Variant19(ast::Statement),
        Variant20(alloc::vec::Vec<ast::Statement>),
        Variant21(ast::TopLevelDecl),
        Variant22(alloc::vec::Vec<ast::TopLevelDecl>),
        Variant23(ast::TopLevelDef),
        Variant24(alloc::vec::Vec<ast::Type>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 7, 54, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 7
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 62, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 10
        0, 7, 67, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 11
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 12
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 13
        0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 19
        0, 7, -37, 38, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0,
        // State 20
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 21
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 22
        0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 39, 49, 0,
        // State 23
        0, 7, -40, 38, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0,
        // State 24
        0, 7, -56, 38, 0, -56, 0, 0, 0, -56, 0, 0, 57, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 25
        0, 7, -56, 38, 0, -56, 0, 0, 0, -56, 0, 0, 57, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 26
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 27
        0, 7, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0,
        // State 36
        0, -79, -79, -79, 0, -79, 0, 0, 0, -79, 0, 0, 0, -79, 0, 0, 0, 0, -79, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 55, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -56, -56, -56, -56, -56, -56, -56, 0, 0, -56, -56, 0, 57, -56, 0, 0, 0, 0, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, 0, 0, 0, -56, 0, 0,
        // State 39
        -51, 14, -51, -51, -51, -51, -51, 0, 0, 0, -51, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        15, 0, -67, 16, -67, -67, -67, 0, 0, 0, 17, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        -93, -93, -93, -93, -93, -93, -93, 0, 0, 0, -93, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, -63, 0, 0, 0, -63, 0, 0, -63, -63, 0,
        // State 44
        0, 0, -45, 0, 18, -45, 19, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, -34, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -94, -94, -94, -94, -94, -94, -94, 0, 0, 0, -94, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -92, -92, -92, -92, -92, -92, -92, 0, 0, 0, -92, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        -57, -57, -57, -57, -57, -57, -57, 0, 0, -57, -57, 0, 65, -57, 0, 0, 0, 0, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, 0, 0, 0, 0, 0, -57, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -90, -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0,
        // State 53
        0, -86, -86, -86, 0, -86, 0, 0, 0, -86, 0, 0, 0, -86, 0, 0, 0, 0, -86, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, 0, 0, 0, -58, 0, 0, -58, -58, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, -64, 0, 0, 0, -64, 0, 0, -64, -64, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, 0, 0, 0, -60, 0, 0, -60, -60, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0,
        // State 65
        0, -91, -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0,
        // State 66
        0, -87, -87, -87, 0, -87, 0, 0, 0, -87, 0, 0, 0, -87, 0, 0, 0, 0, -87, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0,
        // State 67
        0, -80, -80, -80, 0, -80, 0, 0, 0, -80, 0, 0, 0, -80, 0, 0, 0, 0, -80, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0,
        // State 68
        0, -81, -81, -81, 0, -81, 0, 0, 0, -81, 0, 0, 0, -81, 0, 0, 0, 0, -81, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 0,
        // State 70
        0, -84, -84, -84, 0, -84, 0, 0, 0, -84, 0, 0, 0, -84, 0, 0, 0, 0, -84, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0,
        // State 71
        0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, -41, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        -50, 14, -50, -50, -50, -50, -50, 0, 0, 0, -50, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        -48, 14, -48, -48, -48, -48, -48, 0, 0, 0, -48, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        -49, 14, -49, -49, -49, -49, -49, 0, 0, 0, -49, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        15, 0, -65, 16, -65, -65, -65, 0, 0, 0, 17, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        15, 0, -66, 16, -66, -66, -66, 0, 0, 0, 17, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, 0, 0, 0, -59, 0, 0, -59, -59, 0,
        // State 80
        0, 0, 90, 0, 0, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, -36, 0, 0, 92, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, -82, -82, -82, 0, -82, 0, 0, 0, -82, 0, 0, 0, -82, 0, 0, 0, 0, -82, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0,
        // State 83
        0, -83, -83, -83, 0, -83, 0, 0, 0, -83, 0, 0, 0, -83, 0, 0, 0, 0, -83, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, -43, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, -33, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17, 0,
        // State 88
        0, 0, -39, 0, 0, 96, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, -11, -11, -11, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0,
        // State 92
        0, 0, -35, 0, 0, 100, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, -85, -85, -85, 0, -85, 0, 0, 0, -85, 0, 0, 0, -85, 0, 0, 0, 0, -85, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0,
        // State 94
        0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18, 0,
        // State 95
        0, -13, -13, -13, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0,
        // State 96
        0, 0, -38, 0, 0, 101, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, -10, -10, -10, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0,
        // State 100
        0, -12, -12, -12, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 43 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -54,
        // State 1
        -55,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -95,
        // State 29
        -75,
        // State 30
        -72,
        // State 31
        0,
        // State 32
        -76,
        // State 33
        -78,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -31,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -77,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -29,
        // State 59
        0,
        // State 60
        0,
        // State 61
        -32,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        -30,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        -71,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        -69,
        // State 103
        -70,
        // State 104
        0,
        // State 105
        -68,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            5 => 23,
            8 => 22,
            11 => 35,
            14 => match state {
                5 => 51,
                _ => 33,
            },
            15 => match state {
                14 => 73,
                15 => 74,
                16 => 75,
                _ => 39,
            },
            16 => 80,
            17 => 71,
            18 => match state {
                8 => 59,
                9 => 62,
                13 => 72,
                22 => 85,
                _ => 40,
            },
            20 => match state {
                17 => 77,
                18 => 78,
                _ => 41,
            },
            22 => 28,
            23 => match state {
                4 | 8..=9 | 13..=18 | 22 => 42,
                _ => 36,
            },
            24 => match state {
                8 => 60,
                _ => 43,
            },
            26 => 8,
            27 => 44,
            28 => match state {
                1 => 32,
                _ => 29,
            },
            30 => 1,
            31 => 30,
            32 => match state {
                6 => 52,
                7 => 55,
                10 => 65,
                11 => 67,
                12 => 68,
                19 => 81,
                20 => 82,
                21 => 83,
                23 => 88,
                24 => 92,
                25 => 96,
                26 => 101,
                27 => 104,
                _ => 5,
            },
            34 => 10,
            35 => 45,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""%""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""...""###,
            r###""/""###,
            r###"":""###,
            r###""::""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###""=>""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""const""###,
            r###""dyn""###,
            r###""else""###,
            r###""entry""###,
            r###""extern""###,
            r###""fn""###,
            r###""if""###,
            r###""let""###,
            r###""mut""###,
            r###""return""###,
            r###""static""###,
            r###""yield""###,
            r###""{""###,
            r###""}""###,
            r###"BHSTRING"###,
            r###"BSTRING"###,
            r###"CHSTRING"###,
            r###"CSTRING"###,
            r###"FLOAT"###,
            r###"HSTRING"###,
            r###"ID"###,
            r###"INT"###,
            r###"STRING"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexError;
        type Token = lexer::Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Vec<ast::TopLevelDecl>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 43 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Token::Percent if true => Some(0),
            lexer::Token::OpenPar if true => Some(1),
            lexer::Token::ClosePar if true => Some(2),
            lexer::Token::Star if true => Some(3),
            lexer::Token::Plus if true => Some(4),
            lexer::Token::Comma if true => Some(5),
            lexer::Token::Hyphen if true => Some(6),
            lexer::Token::SingleArrow if true => Some(7),
            lexer::Token::Dot if true => Some(8),
            lexer::Token::TplDot if true => Some(9),
            lexer::Token::Slash if true => Some(10),
            lexer::Token::Colon if true => Some(11),
            lexer::Token::DblColon if true => Some(12),
            lexer::Token::Semicolon if true => Some(13),
            lexer::Token::Less if true => Some(14),
            lexer::Token::Equals if true => Some(15),
            lexer::Token::DoubleArrow if true => Some(16),
            lexer::Token::Greater if true => Some(17),
            lexer::Token::OpenBracket if true => Some(18),
            lexer::Token::CloseBracket if true => Some(19),
            lexer::Token::Const if true => Some(20),
            lexer::Token::Dyn if true => Some(21),
            lexer::Token::Else if true => Some(22),
            lexer::Token::Entry if true => Some(23),
            lexer::Token::Extern if true => Some(24),
            lexer::Token::Fn if true => Some(25),
            lexer::Token::If if true => Some(26),
            lexer::Token::Let if true => Some(27),
            lexer::Token::Mut if true => Some(28),
            lexer::Token::Return if true => Some(29),
            lexer::Token::Static if true => Some(30),
            lexer::Token::Yield if true => Some(31),
            lexer::Token::OpenBrace if true => Some(32),
            lexer::Token::CloseBrace if true => Some(33),
            lexer::Token::ByteHeapString(_) if true => Some(34),
            lexer::Token::ByteStaticString(_) if true => Some(35),
            lexer::Token::CHeapString(_) if true => Some(36),
            lexer::Token::CStaticString(_) if true => Some(37),
            lexer::Token::Float(_) if true => Some(38),
            lexer::Token::LokHeapString(_) if true => Some(39),
            lexer::Token::Identifier(_) if true => Some(40),
            lexer::Token::Integer(_) if true => Some(41),
            lexer::Token::LokStaticString(_) if true => Some(42),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 => __Symbol::Variant0(__token),
            34 | 35 | 36 | 37 | 39 | 42 => match __token {
                lexer::Token::ByteHeapString(__tok0) | lexer::Token::ByteStaticString(__tok0) | lexer::Token::CHeapString(__tok0) | lexer::Token::CStaticString(__tok0) | lexer::Token::LokHeapString(__tok0) | lexer::Token::LokStaticString(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            38 | 40 | 41 => match __token {
                lexer::Token::Float(__tok0) | lexer::Token::Identifier(__tok0) | lexer::Token::Integer(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct LokFileParser {
        _priv: (),
    }

    impl LokFileParser {
        pub fn new() -> LokFileParser {
            LokFileParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Vec<ast::TopLevelDecl>, __lalrpop_util::ParseError<usize, lexer::Token, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Vec<ast::TopLevelDecl>,__lalrpop_util::ParseError<usize, lexer::Token, LexError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                // __LokFile = LokFile => ActionFn(0);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (core::option::Option<String>, ast::Type), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(core::option::Option<String>, ast::Type)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::TopLevelDecl>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<u8>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::TopLevelDecl>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::NSIdent, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::TopLevelDecl, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::TopLevelDef, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(core::option::Option<String>, ast::Type)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<lexer::Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // "..."? = "..." => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // "..."? =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("->" <Type>) = "->", Type => ActionFn(42);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("->" <Type>)? = "->", Type => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("->" <Type>)? =  => ActionFn(41);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action41::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",") = ID, Type, "," => ActionFn(82);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",") = Type, "," => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")* = (<(ID? Type)> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")+ = ID, Type, "," => ActionFn(86);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")+ = Type, "," => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")+ = (<(ID? Type)> ",")+, ID, Type, "," => ActionFn(88);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 5)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<(ID? Type)> ",")+ = (<(ID? Type)> ",")+, Type, "," => ActionFn(89);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",") = Expression, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")* = (<Expression> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = Expression, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expression> ",")+ = (<Expression> ",")+, Expression, "," => ActionFn(93);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ID> "::") = ID, "::" => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ID> "::")* =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ID> "::")* = (<ID> "::")+ => ActionFn(31);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ID> "::")+ = ID, "::" => ActionFn(96);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action96::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ID> "::")+ = (<ID> "::")+, ID, "::" => ActionFn(97);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (ID? Type) = ID, Type => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (ID? Type) = Type => ActionFn(81);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (ID? Type)? = ID, Type => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action84::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (ID? Type)? = Type => ActionFn(85);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (ID? Type)? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", Expression, "}" => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", Statement+, Expression, "}" => ActionFn(115);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action115::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", "}" => ActionFn(116);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action116::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", Statement+, "}" => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // CallExpression = CallExpression, "(", Comma<Expression>, ")" => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // CallExpression = ValueExpression => ActionFn(17);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> = ID, Type => ActionFn(100);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action100::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> = Type => ActionFn(101);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> =  => ActionFn(102);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action102::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> = (<(ID? Type)> ",")+, ID, Type => ActionFn(103);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action103::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> = (<(ID? Type)> ",")+, Type => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action104::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<(ID? Type)> = (<(ID? Type)> ",")+ => ActionFn(105);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = Expression => ActionFn(108);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> =  => ActionFn(109);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action109::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+, Expression => ActionFn(110);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action110::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expression> = (<Expression> ",")+ => ActionFn(111);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = SumExpression => ActionFn(8);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? = Expression => ActionFn(36);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression? =  => ActionFn(37);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action37::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorExpression = FactorExpression, "*", CallExpression => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorExpression = FactorExpression, "/", CallExpression => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorExpression = FactorExpression, "%", CallExpression => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorExpression = CallExpression => ActionFn(15);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ID? = ID => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ID? =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LokFile =  => ActionFn(118);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action118::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LokFile = TopLevelDecl+ => ActionFn(119);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action119::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NSIdent = ID => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NSIdent = (<ID> "::")+, ID => ActionFn(99);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action99::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, ";" => ActionFn(6);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "return", Expression, ";" => ActionFn(112);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action112::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "return", ";" => ActionFn(113);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action113::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(39);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(58);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SumExpression = SumExpression, "+", FactorExpression => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SumExpression = SumExpression, "-", FactorExpression => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SumExpression = FactorExpression => ActionFn(11);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl = "extern", "fn", ID, "(", Comma<(ID? Type)>, "...", ")", "->", Type, ";" => ActionFn(74);
        assert!(__symbols.len() >= 10);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant4(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym9.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (10, 28)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl = "extern", "fn", ID, "(", Comma<(ID? Type)>, "...", ")", ";" => ActionFn(75);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action75::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (8, 28)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl = "extern", "fn", ID, "(", Comma<(ID? Type)>, ")", "->", Type, ";" => ActionFn(76);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant4(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (9, 28)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl = "extern", "fn", ID, "(", Comma<(ID? Type)>, ")", ";" => ActionFn(77);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action77::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (7, 28)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl = TopLevelDef => ActionFn(3);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (0, 29)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl* = TopLevelDecl+ => ActionFn(50);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl+ = TopLevelDecl => ActionFn(51);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDecl+ = TopLevelDecl+, TopLevelDecl => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 30)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDef = "entry", "->", Type, Block => ActionFn(78);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action78::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (4, 31)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelDef = "entry", Block => ActionFn(79);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action79::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 31)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = NSIdent => ActionFn(21);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "*", "const", Type => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "*", "mut", Type => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "*", "dyn", "const", Type => ActionFn(24);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce82<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "*", "dyn", "mut", Type => ActionFn(25);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce83<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "[", Type, "]" => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce84<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "[", Type, ";", INT, "]" => ActionFn(27);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 32)
    }
    pub(crate) fn __reduce85<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "(", ")" => ActionFn(120);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action120::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 32)
    }
    pub(crate) fn __reduce86<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = "(", Type+, ")" => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant24(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce87<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type* =  => ActionFn(33);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action33::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 33)
    }
    pub(crate) fn __reduce88<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type* = Type+ => ActionFn(34);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce89<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type+ = Type => ActionFn(63);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce90<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type+ = Type+, Type => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 34)
    }
    pub(crate) fn __reduce91<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValueExpression = INT => ActionFn(18);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce92<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValueExpression = NSIdent => ActionFn(19);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValueExpression = CSTRING => ActionFn(20);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 35)
    }
}
pub use self::__parse__LokFile::LokFileParser;

fn __action0<
>(
    (_, __0, _): (usize, Vec<ast::TopLevelDecl>, usize),
) -> Vec<ast::TopLevelDecl>
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, alloc::vec::Vec<ast::TopLevelDecl>, usize),
) -> Vec<ast::TopLevelDecl>
{
    __0
}

fn __action2<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, params, _): (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    (_, varadic, _): (usize, core::option::Option<lexer::Token>, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, returns, _): (usize, core::option::Option<ast::Type>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    {
		ast::TopLevelDecl::FnExtern(ast::FnExtern {
			name,
			params,
			varadic: varadic.is_some(),
			returns,
		})
	}
}

fn __action3<
>(
    (_, __0, _): (usize, ast::TopLevelDef, usize),
) -> ast::TopLevelDecl
{
    ast::TopLevelDecl::Def(__0)
}

fn __action4<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, returns, _): (usize, core::option::Option<ast::Type>, usize),
    (_, code, _): (usize, ast::Block, usize),
) -> ast::TopLevelDef
{
    {
		ast::TopLevelDef::Entry(ast::Entry {
			returns, code
		})
	}
}

fn __action5<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, statements, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, tail, _): (usize, core::option::Option<ast::Expression>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Block
{
    {
		ast::Block {
			statements,
			tail,
		}
	}
}

fn __action6<
>(
    (_, __0, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Statement
{
    {
		ast::Statement::Expression(__0)
	}
}

fn __action7<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, core::option::Option<ast::Expression>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Statement
{
    {
		ast::Statement::Return(__0)
	}
}

fn __action8<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action9<
>(
    (_, lhs, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, rhs, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Add(Box::new(lhs), Box::new(rhs))
}

fn __action10<
>(
    (_, lhs, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, rhs, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Sub(Box::new(lhs), Box::new(rhs))
}

fn __action11<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action12<
>(
    (_, lhs, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, rhs, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Mul(Box::new(lhs), Box::new(rhs))
}

fn __action13<
>(
    (_, lhs, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, rhs, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Div(Box::new(lhs), Box::new(rhs))
}

fn __action14<
>(
    (_, lhs, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, rhs, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Rem(Box::new(lhs), Box::new(rhs))
}

fn __action15<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action16<
>(
    (_, e, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, p, _): (usize, Vec<ast::Expression>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Expression
{
    ast::Expression::Call(Box::new(e), p)
}

fn __action17<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action18<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Int(str::parse(&__0).unwrap())
}

fn __action19<
>(
    (_, __0, _): (usize, ast::NSIdent, usize),
) -> ast::Expression
{
    ast::Expression::Var(__0)
}

fn __action20<
>(
    (_, __0, _): (usize, Vec<u8>, usize),
) -> ast::Expression
{
    ast::Expression::CStringRef(__0)
}

fn __action21<
>(
    (_, __0, _): (usize, ast::NSIdent, usize),
) -> ast::Type
{
    ast::Type::Name(__0)
}

fn __action22<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    ast::Type::PtrConst(Box::new(__0))
}

fn __action23<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    ast::Type::PtrMut(Box::new(__0))
}

fn __action24<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    ast::Type::PtrDynConst(Box::new(__0))
}

fn __action25<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    ast::Type::PtrDynMut(Box::new(__0))
}

fn __action26<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Type
{
    ast::Type::Slice(Box::new(__0))
}

fn __action27<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, t, _): (usize, ast::Type, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Type
{
    ast::Type::Arr(Box::new(t), str::parse(&n).unwrap())
}

fn __action28<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, alloc::vec::Vec<ast::Type>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Type
{
    ast::Type::Tuple(__0)
}

fn __action29<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ast::NSIdent
{
    {
		v.push(e);
		v
	}
}

fn __action30<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

fn __action31<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

fn __action32<
>(
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> String
{
    __0
}

fn __action33<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Type>
{
    alloc::vec![]
}

fn __action34<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Type>, usize),
) -> alloc::vec::Vec<ast::Type>
{
    v
}

fn __action35<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<ast::Expression>, usize),
    (_, e, _): (usize, core::option::Option<ast::Expression>, usize),
) -> Vec<ast::Expression>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

fn __action36<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> core::option::Option<ast::Expression>
{
    Some(__0)
}

fn __action37<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Expression>
{
    None
}

fn __action38<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Statement>
{
    alloc::vec![]
}

fn __action39<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    v
}

fn __action40<
>(
    (_, __0, _): (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    Some(__0)
}

fn __action41<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Type>
{
    None
}

fn __action42<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    __0
}

fn __action43<
>(
    (_, __0, _): (usize, lexer::Token, usize),
) -> core::option::Option<lexer::Token>
{
    Some(__0)
}

fn __action44<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<lexer::Token>
{
    None
}

fn __action45<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    (_, e, _): (usize, core::option::Option<(core::option::Option<String>, ast::Type)>, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

fn __action46<
>(
    (_, __0, _): (usize, core::option::Option<String>, usize),
    (_, __1, _): (usize, ast::Type, usize),
) -> (core::option::Option<String>, ast::Type)
{
    (__0, __1)
}

fn __action47<
>(
    (_, __0, _): (usize, String, usize),
) -> core::option::Option<String>
{
    Some(__0)
}

fn __action48<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<String>
{
    None
}

fn __action49<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::TopLevelDecl>
{
    alloc::vec![]
}

fn __action50<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::TopLevelDecl>, usize),
) -> alloc::vec::Vec<ast::TopLevelDecl>
{
    v
}

fn __action51<
>(
    (_, __0, _): (usize, ast::TopLevelDecl, usize),
) -> alloc::vec::Vec<ast::TopLevelDecl>
{
    alloc::vec![__0]
}

fn __action52<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::TopLevelDecl>, usize),
    (_, e, _): (usize, ast::TopLevelDecl, usize),
) -> alloc::vec::Vec<ast::TopLevelDecl>
{
    { let mut v = v; v.push(e); v }
}

fn __action53<
>(
    (_, __0, _): (usize, (core::option::Option<String>, ast::Type), usize),
) -> core::option::Option<(core::option::Option<String>, ast::Type)>
{
    Some(__0)
}

fn __action54<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<(core::option::Option<String>, ast::Type)>
{
    None
}

fn __action55<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    alloc::vec![]
}

fn __action56<
>(
    (_, v, _): (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    v
}

fn __action57<
>(
    (_, __0, _): (usize, (core::option::Option<String>, ast::Type), usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> (core::option::Option<String>, ast::Type)
{
    __0
}

fn __action58<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    alloc::vec![__0]
}

fn __action59<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, e, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    { let mut v = v; v.push(e); v }
}

fn __action60<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Expression>
{
    alloc::vec![]
}

fn __action61<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Expression>
{
    v
}

fn __action62<
>(
    (_, __0, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Expression
{
    __0
}

fn __action63<
>(
    (_, __0, _): (usize, ast::Type, usize),
) -> alloc::vec::Vec<ast::Type>
{
    alloc::vec![__0]
}

fn __action64<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Type>, usize),
    (_, e, _): (usize, ast::Type, usize),
) -> alloc::vec::Vec<ast::Type>
{
    { let mut v = v; v.push(e); v }
}

fn __action65<
>(
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

fn __action66<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

fn __action67<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> alloc::vec::Vec<ast::Expression>
{
    alloc::vec![__0]
}

fn __action68<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Expression>, usize),
    (_, e, _): (usize, ast::Expression, usize),
) -> alloc::vec::Vec<ast::Expression>
{
    { let mut v = v; v.push(e); v }
}

fn __action69<
>(
    (_, __0, _): (usize, (core::option::Option<String>, ast::Type), usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    alloc::vec![__0]
}

fn __action70<
>(
    (_, v, _): (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    (_, e, _): (usize, (core::option::Option<String>, ast::Type), usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    { let mut v = v; v.push(e); v }
}

fn __action71<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, lexer::Token, usize),
    __7: (usize, core::option::Option<ast::Type>, usize),
    __8: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action43(
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
    )
}

fn __action72<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, core::option::Option<ast::Type>, usize),
    __7: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action44(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
    )
}

fn __action73<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
    )
}

fn __action74<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, lexer::Token, usize),
    __7: (usize, lexer::Token, usize),
    __8: (usize, ast::Type, usize),
    __9: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __7.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action73(
        __7,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __9,
    )
}

fn __action75<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, lexer::Token, usize),
    __7: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __6.2.clone();
    let __end0 = __7.0.clone();
    let __temp0 = __action41(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __7,
    )
}

fn __action76<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, lexer::Token, usize),
    __7: (usize, ast::Type, usize),
    __8: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __6.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action73(
        __6,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
        __8,
    )
}

fn __action77<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, String, usize),
    __3: (usize, lexer::Token, usize),
    __4: (usize, Vec<(core::option::Option<String>, ast::Type)>, usize),
    __5: (usize, lexer::Token, usize),
    __6: (usize, lexer::Token, usize),
) -> ast::TopLevelDecl
{
    let __start0 = __5.2.clone();
    let __end0 = __6.0.clone();
    let __temp0 = __action41(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
        __6,
    )
}

fn __action78<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
    __2: (usize, ast::Type, usize),
    __3: (usize, ast::Block, usize),
) -> ast::TopLevelDef
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action73(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __temp0,
        __3,
    )
}

fn __action79<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, ast::Block, usize),
) -> ast::TopLevelDef
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action41(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __temp0,
        __1,
    )
}

fn __action80<
>(
    __0: (usize, String, usize),
    __1: (usize, ast::Type, usize),
) -> (core::option::Option<String>, ast::Type)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __temp0,
        __1,
    )
}

fn __action81<
>(
    __0: (usize, ast::Type, usize),
) -> (core::option::Option<String>, ast::Type)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __temp0,
        __0,
    )
}

fn __action82<
>(
    __0: (usize, String, usize),
    __1: (usize, ast::Type, usize),
    __2: (usize, lexer::Token, usize),
) -> (core::option::Option<String>, ast::Type)
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action80(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __temp0,
        __2,
    )
}

fn __action83<
>(
    __0: (usize, ast::Type, usize),
    __1: (usize, lexer::Token, usize),
) -> (core::option::Option<String>, ast::Type)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action81(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __temp0,
        __1,
    )
}

fn __action84<
>(
    __0: (usize, String, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action80(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        __temp0,
    )
}

fn __action85<
>(
    __0: (usize, ast::Type, usize),
) -> core::option::Option<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action81(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        __temp0,
    )
}

fn __action86<
>(
    __0: (usize, String, usize),
    __1: (usize, ast::Type, usize),
    __2: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action82(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __temp0,
    )
}

fn __action87<
>(
    __0: (usize, ast::Type, usize),
    __1: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action83(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __temp0,
    )
}

fn __action88<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    __1: (usize, String, usize),
    __2: (usize, ast::Type, usize),
    __3: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __1.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action82(
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        __0,
        __temp0,
    )
}

fn __action89<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    __1: (usize, ast::Type, usize),
    __2: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action83(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        __0,
        __temp0,
    )
}

fn __action90<
>(
    __0: (usize, core::option::Option<(core::option::Option<String>, ast::Type)>, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action55(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
        __0,
    )
}

fn __action91<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    __1: (usize, core::option::Option<(core::option::Option<String>, ast::Type)>, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
        __1,
    )
}

fn __action92<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<ast::Expression>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action62(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        __temp0,
    )
}

fn __action93<
>(
    __0: (usize, alloc::vec::Vec<ast::Expression>, usize),
    __1: (usize, ast::Expression, usize),
    __2: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<ast::Expression>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action62(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __0,
        __temp0,
    )
}

fn __action94<
>(
    __0: (usize, core::option::Option<ast::Expression>, usize),
) -> Vec<ast::Expression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action60(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __0,
    )
}

fn __action95<
>(
    __0: (usize, alloc::vec::Vec<ast::Expression>, usize),
    __1: (usize, core::option::Option<ast::Expression>, usize),
) -> Vec<ast::Expression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action61(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __1,
    )
}

fn __action96<
>(
    __0: (usize, String, usize),
    __1: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
    )
}

fn __action97<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, lexer::Token, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action32(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        __0,
        __temp0,
    )
}

fn __action98<
>(
    __0: (usize, String, usize),
) -> ast::NSIdent
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action30(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __temp0,
        __0,
    )
}

fn __action99<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> ast::NSIdent
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action31(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __temp0,
        __1,
    )
}

fn __action100<
>(
    __0: (usize, String, usize),
    __1: (usize, ast::Type, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action84(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        __temp0,
    )
}

fn __action101<
>(
    __0: (usize, ast::Type, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action85(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        __temp0,
    )
}

fn __action102<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        __temp0,
    )
}

fn __action103<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    __1: (usize, String, usize),
    __2: (usize, ast::Type, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action84(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        __0,
        __temp0,
    )
}

fn __action104<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
    __1: (usize, ast::Type, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action85(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        __0,
        __temp0,
    )
}

fn __action105<
>(
    __0: (usize, alloc::vec::Vec<(core::option::Option<String>, ast::Type)>, usize),
) -> Vec<(core::option::Option<String>, ast::Type)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        __0,
        __temp0,
    )
}

fn __action106<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    __2: (usize, ast::Expression, usize),
    __3: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action36(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __temp0,
        __3,
    )
}

fn __action107<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __temp0,
        __2,
    )
}

fn __action108<
>(
    __0: (usize, ast::Expression, usize),
) -> Vec<ast::Expression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        __temp0,
    )
}

fn __action109<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::Expression>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        __temp0,
    )
}

fn __action110<
>(
    __0: (usize, alloc::vec::Vec<ast::Expression>, usize),
    __1: (usize, ast::Expression, usize),
) -> Vec<ast::Expression>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action36(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        __0,
        __temp0,
    )
}

fn __action111<
>(
    __0: (usize, alloc::vec::Vec<ast::Expression>, usize),
) -> Vec<ast::Expression>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        __0,
        __temp0,
    )
}

fn __action112<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, ast::Expression, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Statement
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action36(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __0,
        __temp0,
        __2,
    )
}

fn __action113<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
) -> ast::Statement
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __0,
        __temp0,
        __1,
    )
}

fn __action114<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, ast::Expression, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action38(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        __0,
        __temp0,
        __1,
        __2,
    )
}

fn __action115<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    __2: (usize, ast::Expression, usize),
    __3: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        __0,
        __temp0,
        __2,
        __3,
    )
}

fn __action116<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action38(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        __0,
        __temp0,
        __1,
    )
}

fn __action117<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        __0,
        __temp0,
        __2,
    )
}

fn __action118<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::TopLevelDecl>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action119<
>(
    __0: (usize, alloc::vec::Vec<ast::TopLevelDecl>, usize),
) -> Vec<ast::TopLevelDecl>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action50(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action120<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
) -> ast::Type
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action33(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __0,
        __temp0,
        __1,
    )
}

fn __action121<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Type>, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Type
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __0,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize), __lalrpop_util::ParseError<usize, lexer::Token, LexError>>;
}

impl<> __ToTriple<> for (usize, lexer::Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize), __lalrpop_util::ParseError<usize, lexer::Token, LexError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, lexer::Token, usize), LexError> {
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize), __lalrpop_util::ParseError<usize, lexer::Token, LexError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
