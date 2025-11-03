/// POSIX/GNU command-line argument parser - Rust translation
/// This file matches carg_parser.c structure exactly for human review
/// C source: carg_parser.c (306 lines, 9,572 bytes) - IMMUTABLE REFERENCE

// Vec is in std::vec, not std::collections

/// Argument parsing option types - matches carg_parser.h ap_Has_arg enum
#[derive(Clone, Copy, PartialEq)]
pub enum ApHasArg {
    ApNo = 0,      // Option has no argument
    ApYes = 1,     // Option requires an argument
    ApMaybe = 2,   // Option has optional argument
    ApYesme = 3,   // Option requires argument and merges
}

/// Option specification - matches carg_parser.h ap_Option struct
#[derive(Clone)]
pub struct ApOption {
    pub code: i32,               // Option code
    pub name: Option<String>,    // Long option name (null for short-only)
    pub has_arg: ApHasArg,       // Argument requirement type
}

/// Parsed argument record - matches carg_parser.h ap_Record struct
#[derive(Clone)]
struct ApRecord {
    code: i32,                    // Option code (0 for non-option arguments)
    parsed_name: Option<String>,  // Name as parsed (with dashes)
    argument: Option<String>,     // Option argument
}

/// Argument parser state - matches carg_parser.h Arg_parser struct
pub struct ArgParser {
    data: Vec<ApRecord>,          // Array of parsed records
    error: Option<String>,        // Error message (if any)
}

impl ArgParser {
    /// Create new argument parser - constructor equivalent
    pub fn new() -> Self {
        ArgParser {
            data: Vec::new(),
            error: None,
        }
    }
}

/// ap_resize_buffer - matches carg_parser.c:27
fn ap_resize_buffer<T>(buf: &mut Vec<T>, min_size: usize) {
    if buf.capacity() < min_size {
        buf.reserve(min_size - buf.capacity());
    }
}

/// set_argument - matches carg_parser.c:35
fn set_argument(ap: &mut ArgParser, argument: &str) -> bool {
    if let Some(last_record) = ap.data.last_mut() {
        last_record.argument = Some(argument.to_string());
        true
    } else {
        false
    }
}

/// push_back_record - matches carg_parser.c:46
fn push_back_record(ap: &mut ArgParser) -> bool {
    ap.data.push(ApRecord {
        code: 0,
        parsed_name: None,
        argument: None,
    });
    true
}

/// push_back_option - matches carg_parser.c:56
fn push_back_option(ap: &mut ArgParser, code: i32, long_name: Option<&str>) -> bool {
    if !push_back_record(ap) {
        return false;
    }
    
    if let Some(last_record) = ap.data.last_mut() {
        last_record.code = code;
        
        if let Some(name) = long_name {
            // Format as "--name"
            last_record.parsed_name = Some(format!("--{}", name));
        } else {
            // Format as "-X" where X is the character code
            if code >= 0 && code <= 255 {
                last_record.parsed_name = Some(format!("-{}", char::from(code as u8)));
            } else {
                last_record.parsed_name = Some(format!("-{}", code));
            }
        }
    }
    true
}

/// push_back_argument - matches carg_parser.c:81
fn push_back_argument(ap: &mut ArgParser, argument: &str) -> bool {
    if !push_back_record(ap) {
        return false;
    }
    
    if let Some(last_record) = ap.data.last_mut() {
        last_record.code = 0;
        last_record.parsed_name = None;
        last_record.argument = Some(argument.to_string());
    }
    true
}

/// set_error - matches carg_parser.c:92
fn set_error(ap: &mut ArgParser, s1: &str, s2: &str, s3: &str) -> bool {
    ap.error = Some(format!("{}{}{}", s1, s2, s3));
    false
}

/// free_data - matches carg_parser.c:106
fn free_data(ap: &mut ArgParser) {
    ap.data.clear();
}

/// parse_long_option - matches carg_parser.c:117
fn parse_long_option(
    ap: &mut ArgParser,
    opt: &str,
    arg: Option<&str>,
    options: &[ApOption],
    argindp: &mut usize,
) -> bool {
    // Remove leading "--"
    let opt_name = if opt.starts_with("--") {
        &opt[2..]
    } else {
        return set_error(ap, "invalid long option '", opt, "'");
    };
    
    // Check for embedded argument (--option=value)
    let (option_name, embedded_arg) = if let Some(eq_pos) = opt_name.find('=') {
        let name = &opt_name[..eq_pos];
        let arg_val = &opt_name[eq_pos + 1..];
        (name, Some(arg_val))
    } else {
        (opt_name, None)
    };
    
    // Find matching option
    for option in options {
        if let Some(ref name) = option.name {
            if name == option_name {
                if !push_back_option(ap, option.code, Some(name)) {
                    return false;
                }
                
                // Handle argument based on option requirements
                match option.has_arg {
                    ApHasArg::ApNo => {
                        if embedded_arg.is_some() {
                            return set_error(ap, "option doesn't allow an argument -- '", option_name, "'");
                        }
                        *argindp += 1;
                    }
                    ApHasArg::ApYes | ApHasArg::ApYesme => {
                        let arg_to_use = embedded_arg.or(arg);
                        if let Some(argument) = arg_to_use {
                            if option.has_arg == ApHasArg::ApYes && argument.is_empty() {
                                return set_error(ap, "option requires an argument -- '", option_name, "'");
                            }
                            if !set_argument(ap, argument) {
                                return false;
                            }
                            if embedded_arg.is_none() {
                                *argindp += 1; // Skip the argument
                            }
                        } else {
                            return set_error(ap, "option requires an argument -- '", option_name, "'");
                        }
                        *argindp += 1;
                    }
                    ApHasArg::ApMaybe => {
                        if let Some(argument) = embedded_arg {
                            if !set_argument(ap, argument) {
                                return false;
                            }
                        }
                        *argindp += 1;
                    }
                }
                return true;
            }
        }
    }
    
    set_error(ap, "unrecognized option '", opt, "'")
}

/// parse_short_option - matches carg_parser.c:175
fn parse_short_option(
    ap: &mut ArgParser,
    opt: &str,
    arg: Option<&str>,
    options: &[ApOption],
    argindp: &mut usize,
) -> bool {
    let opt_chars: Vec<char> = opt.chars().collect();
    if opt_chars.len() < 2 || opt_chars[0] != '-' {
        return set_error(ap, "invalid short option '", opt, "'");
    }
    
    let mut cind = 1; // Start after the '-'
    
    while cind < opt_chars.len() {
        let ch = opt_chars[cind];
        let code = ch as i32;
        
        // Find matching option
        let mut found = false;
        for option in options {
            if option.code == code && option.name.is_none() {
                found = true;
                if !push_back_option(ap, code, None) {
                    return false;
                }
                
                // Handle argument based on option requirements
                match option.has_arg {
                    ApHasArg::ApNo => {
                        cind += 1;
                    }
                    ApHasArg::ApYes | ApHasArg::ApYesme => {
                        // If there are more characters, use them as argument
                        if cind + 1 < opt_chars.len() {
                            let argument: String = opt_chars[cind + 1..].iter().collect();
                            if !set_argument(ap, &argument) {
                                return false;
                            }
                            *argindp += 1;
                            return true; // Done with this option string
                        } else if let Some(argument) = arg {
                            if option.has_arg == ApHasArg::ApYes && argument.is_empty() {
                                return set_error(ap, "option requires an argument -- '", &ch.to_string(), "'");
                            }
                            if !set_argument(ap, argument) {
                                return false;
                            }
                            *argindp += 1; // Skip the argument
                            *argindp += 1; // Skip the option
                            return true;
                        } else {
                            return set_error(ap, "option requires an argument -- '", &ch.to_string(), "'");
                        }
                    }
                    ApHasArg::ApMaybe => {
                        if cind + 1 < opt_chars.len() {
                            let argument: String = opt_chars[cind + 1..].iter().collect();
                            if !set_argument(ap, &argument) {
                                return false;
                            }
                            *argindp += 1;
                            return true;
                        }
                        cind += 1;
                    }
                }
                break;
            }
        }
        
        if !found {
            return set_error(ap, "invalid option -- '", &ch.to_string(), "'");
        }
    }
    
    *argindp += 1;
    true
}

/// ap_init - matches carg_parser.c:217
pub fn ap_init(
    ap: &mut ArgParser,
    args: &[String],
    options: &[ApOption],
    in_order: bool,
) -> bool {
    let mut non_options: Vec<String> = Vec::new();
    let mut argind = 1; // Start from index 1 (skip program name)
    
    ap.data.clear();
    ap.error = None;
    
    if args.len() < 2 {
        return true; // No arguments to parse
    }
    
    while argind < args.len() {
        let arg = &args[argind];
        
        if arg.starts_with('-') && arg.len() > 1 {
            // Found an option
            let next_arg = if argind + 1 < args.len() {
                Some(args[argind + 1].as_str())
            } else {
                None
            };
            
            if arg == "--" {
                // End of options marker
                argind += 1;
                break;
            } else if arg.starts_with("--") {
                // Long option
                if !parse_long_option(ap, arg, next_arg, options, &mut argind) {
                    free_data(ap);
                    return false;
                }
            } else {
                // Short option(s)
                if !parse_short_option(ap, arg, next_arg, options, &mut argind) {
                    free_data(ap);
                    return false;
                }
            }
            
            if ap.error.is_some() {
                break;
            }
        } else if in_order {
            // Process non-option argument immediately
            if !push_back_argument(ap, arg) {
                return false;
            }
            argind += 1;
        } else {
            // Collect non-option argument for later processing
            non_options.push(arg.clone());
            argind += 1;
        }
    }
    
    // Handle errors
    if ap.error.is_some() {
        free_data(ap);
        return true; // Error was set, but function succeeded in parsing
    }
    
    // Add collected non-options
    for non_opt in &non_options {
        if !push_back_argument(ap, non_opt) {
            return false;
        }
    }
    
    // Add remaining arguments after "--"
    while argind < args.len() {
        if !push_back_argument(ap, &args[argind]) {
            return false;
        }
        argind += 1;
    }
    
    true
}

/// ap_free - matches carg_parser.c:274
pub fn ap_free(ap: &mut ArgParser) {
    free_data(ap);
    ap.error = None;
}

/// ap_error - matches carg_parser.c:281
pub fn ap_error(ap: &ArgParser) -> Option<&str> {
    ap.error.as_deref()
}

/// ap_arguments - matches carg_parser.c:283
pub fn ap_arguments(ap: &ArgParser) -> usize {
    ap.data.len()
}

/// ap_code - matches carg_parser.c:285
pub fn ap_code(ap: &ArgParser, i: usize) -> i32 {
    if i >= ap.data.len() {
        0
    } else {
        ap.data[i].code
    }
}

/// ap_parsed_name - matches carg_parser.c:292
pub fn ap_parsed_name(ap: &ArgParser, i: usize) -> &str {
    if i >= ap.data.len() {
        ""
    } else {
        ap.data[i].parsed_name.as_deref().unwrap_or("")
    }
}

/// ap_argument - matches carg_parser.c:299
pub fn ap_argument(ap: &ArgParser, i: usize) -> Option<&str> {
    if i >= ap.data.len() {
        None
    } else {
        ap.data[i].argument.as_deref()
    }
}