use std::{ptr::null_mut};

use widestring::{U32String, u32str};

use fribidi_sys::fribidi_bindings;

pub struct Fribidi;

#[repr(u32)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum ParagraphType
{
    LeftToRight     = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_LTR,
    RightToLeft     = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_RTL,
    OtherNeutral    = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_ON,
    WeakLeftToRight = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_WLTR,
    WeakRightToLeft = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_WRTL,
}

#[repr(u32)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum CharType
{
    LeftToRight              = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LTR,
    RightToLeft              = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RTL,
    ArabicLetter             = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_AL,
    EuropeanNumeral          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_EN,
    ArabicNumeral            = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_AN,
    EuropeanNumberSeparator  = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ES,
    EuropeanNumberTerminator = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ET,
    CommonSeparator          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_CS,
    NonSpacingMark           = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_NSM,
    BoundaryNeutral          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_BN,
    BlockSeparator           = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_BS,
    SegmentSeparator         = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_SS,
    WhiteSpace               = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_WS,
    OtherNeutral             = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ON,
    LeftToRightEmbedding     = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRE,
    RightToLeftEmbedding     = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLE,
    LeftToRightOverride      = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRO,
    RightToLeftOverride      = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLO,
    PopDirectionalFlag       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_PDF,
    LeftToRightIsolate       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRI,
    RightToLeftIsolate       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLI,
    FirstStongIsolate        = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_FSI,
    PopDirectionalIsolate    = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_PDI,
}

impl Fribidi
{
    /// fribidi_remove_bidi_marks - remove bidi marks out of an string
    ///
    /// This function removes the bidi and boundary-neutral marks out of an string
    /// and the accompanying lists.  It implements rule X9 of the Unicode
    /// Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#X9, with the exception that it removes
    /// U+200E LEFT-TO-RIGHT MARK and U+200F RIGHT-TO-LEFT MARK too.
    ///
    /// If any of the input lists are NULL, the list is skipped.  If str is the
    /// visual string, then positions_to_this is  positions_L_to_V and
    /// position_from_this_list is positions_V_to_L;  if str is the logical
    /// string, the other way. Moreover, the position maps should be filled with
    /// valid entries.
    /// 
    /// A position map pointing to a removed character is filled with \(mi1. By the
    /// way, you should not use embedding_levels if str is visual string.
    /// 
    /// For best results this function should be run on a whole paragraph, not
    /// lines; but feel free to do otherwise if you know what you are doing.
    ///
    /// Returns: New length of the string, or \(mi1 if an error occurred (memory
    /// allocation failure most probably).
    ///
    pub fn remove_bidi_marks<'a>(
        input_str: &'a mut U32String,
        positions_to_this: Option<&mut Vec<i32>>,
        position_from_this_list: Option<&mut Vec<i32>>,
        embedding_levels: Option<&mut Vec<i8>>
    ) -> Result<&'a U32String, String>
    {
        let result_string_len = unsafe {
            fribidi_bindings::fribidi_remove_bidi_marks(
                input_str.as_mut_ptr(),
                input_str.len() as i32,
                if let Some(positions) = positions_to_this {positions.as_mut_ptr()} else {null_mut()},
                if let Some(positions) = position_from_this_list {positions.as_mut_ptr()} else {null_mut()},
                if let Some(levels) = embedding_levels {levels.as_mut_ptr()} else {null_mut()}
            )
        };

        input_str.replace_range(result_string_len as usize.., u32str!(""));

        match result_string_len
        {
            -1 => Err("memory allocation failed".to_owned()),
            _ => Ok(input_str)
        }
    }

    /// fribidi_log2vis - get visual string
    ///
    /// This function converts the logical input string to the visual output
    /// strings as specified by the Unicode Bidirectional Algorithm.  As a side
    /// effect it also generates mapping lists between the two strings, and the
    /// list of embedding levels as defined by the algorithm.
    ///
    /// If NULL is passed as any of the the lists, the list is ignored and not
    /// filled.
    ///
    /// Note that this function handles one-line paragraphs. For multi-
    /// paragraph texts it is necessary to first split the text into
    /// separate paragraphs and then carry over the resolved pbase_dir
    /// between the subsequent invocations.
    ///
    /// Returns: Maximum level found plus one, or zero if any error occurred
    /// (memory allocation failure most probably).
    ///
    pub fn log2vis(
        input_str: &U32String,
        pbase_dir: ParagraphType,
        positions_l_to_v: Option<&mut Vec<i32>>,
        positions_v_to_l: Option<&mut Vec<i32>>,
        embedding_levels: Option<&mut Vec<i8>>
    ) -> Result<(U32String, i8), String>
    {
        let mut visual_str = std::iter::repeat(" ").take(input_str.len()).collect::<U32String>();

        let maximum_level = unsafe {
            fribidi_bindings::fribidi_log2vis(
                input_str.as_ptr(),
                input_str.len() as i32,
                &mut (pbase_dir as u32),
                visual_str.as_mut_ptr(),
                if let Some(positions) = positions_l_to_v {positions.as_mut_ptr()} else {null_mut()},
                if let Some(positions) = positions_v_to_l {positions.as_mut_ptr()} else {null_mut()},
                if let Some(levels) = embedding_levels {levels.as_mut_ptr()} else {null_mut()}
            )
        };

        match maximum_level
        {
            0 => Err("memory allocation failed".to_owned()),
            _ => Ok((visual_str, maximum_level))
        }
    }

    /// fribidi_get_bidi_type - get character bidi type
    ///
    /// This function returns the bidi type of a character as defined in Table 3.7
    /// Bidirectional Character Types of the Unicode Bidirectional Algorithm
    /// available at
    /// http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types, using
    /// data provided in file UnicodeData.txt of the Unicode Character Database
    /// available at http://www.unicode.org/Public/UNIDATA/UnicodeData.txt.
    ///
    /// There are a few macros defined in fribidi-bidi-types.h for querying a bidi
    /// type.
    ///
    pub fn get_bidi_type (input_char: char) -> CharType
    {
        let char_type = unsafe {
            std::mem::transmute (
                fribidi_bindings::fribidi_get_bidi_type(input_char as u32)
            )
        };

        char_type        
    }

    /// fribidi_get_bidi_types - get bidi types for an string of characters
    ///
    /// This function finds the bidi types of an string of characters.  See
    /// fribidi_get_bidi_type() for more information about the bidi types returned
    /// by this function.
    ///
    pub fn get_bidi_types (input_str: &U32String) -> Vec<CharType>
    {
        let mut res: Vec<u32> = vec![0;input_str.len()];
        unsafe {
            fribidi_bindings::fribidi_get_bidi_types(
                input_str.as_ptr(),
                input_str.len() as i32,
                res.as_mut_ptr()
            );

            res.iter_mut().map(|ch| std::mem::transmute(*ch)).collect()
        }
    }

    /// fribidi_get_bidi_type_name - get bidi type name
    ///
    /// This function returns the bidi type name of a character type.
    ///
    /// The type names are the same as ones defined in Table 3.7 Bidirectional
    /// Character Types of the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types, with a
    /// few modifications: L->LTR, R->RTL, B->BS, S->SS.
    ///
    pub fn get_bidi_type_name (char_type: CharType) -> String
    {
        // unsafe {
        //     let bidi_type_name = fribidi_bindings::fribidi_get_bidi_type_name(char_type as u32);
        //     std::ffi::CStr::from_ptr(bidi_type_name).to_str().unwrap()
        // }

        format!("{:?}", char_type)
    }

    /// fribidi_get_par_direction - get base paragraph direction
    ///
    /// This function finds the base direction of a single paragraph,
    /// as defined by rule P2 of the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#P2.
    ///
    /// You typically do not need this function as
    /// fribidi_get_par_embedding_levels() knows how to compute base direction
    /// itself, but you may need this to implement a more sophisticated paragraph
    /// direction handling.  Note that you can pass more than a paragraph to this
    /// function and the direction of the first non-neutral paragraph is returned,
    /// which is a very good heuristic to set direction of the neutral paragraphs
    /// at the beginning of text.  For other neutral paragraphs, you better use the
    /// direction of the previous paragraph.
    ///
    /// Returns: Base pargraph direction.  No weak paragraph direction is returned,
    /// only LeftToRight, RightToLeft, or OtherNeutral.
    ///
    pub fn get_par_direction (char_types: &Vec<CharType>) -> ParagraphType
    {
        let par_direction = unsafe {
            // let bidi_type_name = fribidi_bindings::fribidi_get_bidi_type_name(char_type as u32);
            // fribidi_bindings::fribidi_get_par_direction(bidi_type_name, len);
            fribidi_bindings::fribidi_get_par_direction(char_types.as_ptr() as *const u32, char_types.len() as i32)
        };

        unsafe { std::mem::transmute (par_direction) }

    }
}

#[cfg(test)]
mod test
{
    use widestring::U32String;

    use super::{Fribidi, ParagraphType, CharType};

    #[test]
    fn test_remove_bidi_marks()
    {
        #[allow(text_direction_codepoint_in_literal)]
        let mut text = U32String::from("أحمد‫ خالد");
        let gt = U32String::from("أحمد خالد");

        let mut positions_to_this :Vec<i32> = vec![0; text.len()];
        let mut position_from_this_list :Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(
            Fribidi::remove_bidi_marks(&mut text, Some(&mut positions_to_this), Some(&mut position_from_this_list), None),
            Ok(&gt)
        );

        assert_eq!(positions_to_this, vec![0, 1, 2, 3, -1, 4, 5, 6, 7, 9]);
        assert_eq!(position_from_this_list, vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 9]);
    }

    #[test]
    fn test_log2vis()
    {
        let text = U32String::from("\u{0686}\u{0631}\u{0645}\u{0647}\u{064A}\u{0646}");
        let gt = (U32String::from("\u{FEE6}\u{FEF4}\u{FEEC}\u{FEE3}\u{FEAE}\u{FB7C}"), 2);
        let gt_positions_l_to_v = vec![5, 4, 3, 2, 1, 0];
        let gt_positions_v_to_l = vec![5, 4, 3, 2, 1, 0];
        let gt_embedding_levels = vec![1; 6];

        let mut positions_l_to_v :Vec<i32> = vec![1; text.len()];
        let mut positions_v_to_l :Vec<i32> = vec![1; text.len()];
        let mut embedding_levels: Vec<i8> = vec![1; text.len()];

        let (res, maximum_level) = Fribidi::log2vis(
            &text,
            ParagraphType::RightToLeft,
            Some(&mut positions_l_to_v),
            Some(&mut positions_v_to_l),
            Some(&mut embedding_levels)
        ).unwrap();

        assert_eq!((res, maximum_level), gt);
        assert_eq!(positions_l_to_v, gt_positions_l_to_v);
        assert_eq!(positions_v_to_l, gt_positions_v_to_l);
        assert_eq!(embedding_levels, gt_embedding_levels);
    }

    #[test]
    fn test_get_bidi_type ()
    {
        let ch = 'غ';
        let ch_type = Fribidi::get_bidi_type(ch);
        let gt = CharType::ArabicLetter;
        
        assert_eq!(ch_type, gt);
    }

    #[test]
    fn test_get_bidi_types ()
    {
        let text = U32String::from("غ!A西Б1٤");
        let types = Fribidi::get_bidi_types(&text);
        let gt = vec![
            CharType::ArabicLetter,
            CharType::OtherNeutral,
            CharType::LeftToRight,
            CharType::LeftToRight,
            CharType::LeftToRight,
            CharType::EuropeanNumeral,
            CharType::ArabicNumeral
        ];
        
        assert_eq!(types, gt);
    }

    #[test]
    fn test_get_bidi_type_name ()
    {
        let char_type = CharType::EuropeanNumberSeparator;
        let gt = "EuropeanNumberSeparator".to_owned();

        let char_type_name = Fribidi::get_bidi_type_name(char_type);
        assert_eq!(char_type_name, gt);
    }

    #[test]
    fn test_get_par_direction ()
    {
        let char_types = vec![CharType::ArabicLetter, CharType::ArabicNumeral, CharType::LeftToRight, CharType::LeftToRight];
        let gt = ParagraphType::RightToLeft;

        let par_dir = Fribidi::get_par_direction(&char_types);

        assert_eq!(par_dir, gt);
    }
}