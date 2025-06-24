#[macro_export]
macro_rules! location {
    () => {{
        let location = format!(
            "{}{}{}:{}",
            crate::color::auto($crate::function_name!()),
            crate::color::fore(" @ ", 220),
            crate::color::auto($crate::filename!()),
            crate::color::auto(line!().to_string())
        );
        location
    }};
    (begin) => {
        $crate::tag!([crate::color::auto(format!("in function")), $crate::location!()].join(" "))
    };
    (end) => {
        $crate::tag!([crate::color::auto(format!("from function")), $crate::location!()].join(" "))
    };
    (unexpected) => {
        [
            crate::color::fore(format!("<unexpected branch in function"), 160),
            $crate::location!(),
            crate::color::fore(format!(">"), 160),
        ]
        .join(" ")
    };
}
#[macro_export]
macro_rules! filename {
    () => {{
        let mut parts = file!()
            .split(std::path::MAIN_SEPARATOR_STR)
            .map(String::from)
            .collect::<Vec<String>>();
        let (folder, filename) = if parts.len() > 1 {
            let last = crate::color::auto(parts.remove(parts.len() - 1));
            let parts = parts.iter().map(|part| crate::color::auto(part)).collect::<Vec<String>>();
            (parts, last)
        } else {
            (Vec::<String>::new(), crate::color::auto(parts[0].to_string()))
        };
        if folder.len() > 1 {
            format!(
                "{}{}{}",
                crate::color::auto(filename),
                crate::color::fore(" in ", 7),
                folder.join(std::path::MAIN_SEPARATOR_STR)
            )
        } else {
            crate::color::auto(filename)
        }
    }};
}
#[macro_export]
macro_rules! tag {
    ($arg:expr) => {{
        $crate::tag!($arg, 7)
    }};
    (close, $arg:expr) => {{
        $crate::tag!(close, $arg, 7)
    }};
    ($arg:expr, $color:literal) => {{
        format!(
            "{}{}{}",
            crate::color::fore("<", $color),
            crate::color::auto($arg),
            crate::color::fore(">", $color),
        )
    }};
    (close, $arg:expr, $color:literal) => {{
        format!(
            "{}{}{}",
            crate::color::fore("</", $color),
            $arg,
            crate::color::fore(">", $color),
        )
    }};
}
#[macro_export]
macro_rules! dbg {
    ($( $arg:expr ),* $(,)? ) => {{
        let obj = format!("{}", [$(
            $crate::indent!(format!("{} = {:#?}", stringify!($arg), $arg)),
        )*].iter().map(crate::color::reset).collect::<Vec<String>>().join("\n"));
        eprintln!("{}", crate::color::reset([$crate::location!(begin), obj, $crate::location!(end)].join("\n")));
    }};
}
#[macro_export]
macro_rules! indent {
    ($indentation:literal, $obj:expr) => {{
        format!("{}", $obj)
            .lines()
            .map(|line| format!("{}{}", " ".repeat($indentation), line))
            .collect::<Vec<String>>()
            .join("\n")
    }};
    ($obj:expr) => {{
        $crate::indent!(4, $obj)
    }};
}
#[macro_export]
macro_rules! indent_objdump {
    ($indentation:literal, $obj:expr) => {{
        format!("{:#?}", $obj)
            .lines()
            .map(|line| format!("{}{}", " ".repeat($indentation), line))
            .collect::<Vec<String>>()
            .join("\n")
    }};
    ($obj:expr) => {{
        $crate::indent_objdump!(4, $obj)
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name
            .strip_suffix("::f")
            .unwrap()
            .replace(format!("{}::", module_path!()).as_str(), "");
        name
    }};
}

#[macro_export]
macro_rules! unexpected {
    ($( $arg:expr ),* ) => {{
        $(
            let obj = format!("{:#?}", $arg);
            eprintln!("{}", crate::color::reset([obj, $crate::location!(unexpected)].join(" ")));
        )*
        std::process::exit(107);
    }};
    () => {
        $crate::unexpected!("reach");
    };
}
#[macro_export]
macro_rules! caller {
    () => {
        $crate::Caller($crate::function_name!().to_string(), file!().to_string(), line!())
    };
}

#[macro_export]
macro_rules! with_caller {
    ($error:expr) => {{
        use crate::Traceback;
        $error.with($crate::caller!())
    }};
}

#[macro_export]
macro_rules! map_call_to_result {
    ($result:expr) => {
        $result.map_err(|error| $crate::with_caller!(crate::Error::from(error)))
    };
}
#[macro_export]
macro_rules! try_result {
    ($result:expr) => {
        crate::map_call_to_result!($result)?
    };
}

#[macro_export]
macro_rules! unwrap_result {
    ($result:expr) => {{
        use crate::Traceback;
        $crate::map_call_to_result!($result).unwrap()
    }};
}

#[macro_export]
macro_rules! impl_error {
    ($name:ident, $type:ty) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct Error {
            message: String,
            ty: $type,
            callers: Vec<crate::Caller>,
            previous: Option<Box<Error>>,
        }
        impl Error {
            pub fn new<T: std::fmt::Display>(message: T, ty: $type) -> Self {
                Self::with_previous_error(message, ty, None)
            }

            pub fn with_previous_error<T: std::fmt::Display>(
                message: T,
                ty: $type,
                previous: Option<Error>,
            ) -> Self {
                let message = message.to_string();
                Error {
                    message,
                    ty,
                    callers: Vec::new(),
                    previous: previous.map(Box::new),
                }
            }
        }
        impl std::error::Error for $name {}

        impl $crate::Traceback for $name {
            fn message(&self) -> String {
                self.message.to_string()
            }

            fn callers(&self) -> Vec<$crate::Caller> {
                self.callers.to_vec()
            }

            fn with(&self, caller: $crate::Caller) -> Self {
                let mut error = self.clone();
                error.callers.insert(0, caller);
                error
            }

            fn previous_as_debug(&self) -> String {
                self.previous.clone().map(|error| format!("{:#?}", error)).unwrap_or_default()
            }

            fn previous_as_string(&self) -> String {
                self.previous.clone().map(|error| format!("{}", error)).unwrap_or_default()
            }
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}\n\nreason: {}", self.ty, self.highlight_message())
            }
        }
        impl std::fmt::Debug for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let ty = self.ty.to_string();
                let source = self.to_string();
                write!(
                    f,
                    "{}{}",
                    if ty == source {
                        ty.to_string()
                    } else {
                        format!("{} in source:\n{}", ty, source)
                    },
                    if self.callers.len() > 0 {
                        format!(
                            "\n\nStacktrace:\n{}\n",
                            [self.previous_as_debug(), self.callers_to_string(4)]
                                .iter()
                                .filter(|s| !s.trim().is_empty())
                                .map(String::from)
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    } else {
                        String::new()
                    }
                )
            }
        }
        pub type Result<T> = std::result::Result<T, Error>;
        #[macro_export]
        macro_rules! try_result {
            ($result: expr) => {
                $crate::map_call_to_result!($result)?
            };
        }
        #[macro_export]
        macro_rules! map_call_to_result {
            ($result: expr) => {
                use crate::Traceback;
                $result.map_err(|error| crate::with_caller!(crate::Error::from(error)))
            };
        }
    };
}

#[macro_export]
macro_rules! format_to_str {
    (&$lifetime:lifetime $text:literal, $( $arg:expr ),* $(,)? ) => {
        std::borrow::Cow::from(format!($text, $($arg,)*).as_str())
    };
}

#[macro_export]
macro_rules! vec_deque {
    ($( $arg:expr ),* $(,)? ) => {{
        let mut deque = std::collections::VecDeque::new();
        $(deque.push_back($arg);
        )*
        deque
    }};
}

#[macro_export]
macro_rules! step {
    ($text:literal) => {{
        $crate::step!(length=80, $text)
    }};
    (fg=$fg:literal, $text:literal) => {{
        $crate::step!(bg=$fg, fg=crate::color::invert_bw($fg), length=80, $text)
    }};
    (bg=$bg:expr, fg=$fg:expr, $text:literal) => {{
        $crate::step!(bg=$bg, fg=$fg, length=80, $text)
    }};
    (length=$length:literal, $text:expr) => {{
        let (bg, fg) = crate::color::couple(line!() as usize);
        let text = $text.to_string();
        let bar = crate::color::ansi(
            " ".repeat($length),
            fg.into(),
            bg.into(),
        );
        eprintln!(
            "\n{}",
            [
                bar.clone(),
                crate::color::ansi(
                    if text.is_empty() { String::new() } else { format!("{}", text) },
                    fg.into(),
                    bg.into(),
                ),
            ].join("\n")
        );

    }};
    (bg=$bg:expr, fg=$fg:expr, length=$length:literal, $text:expr) => {{
        let text = $text.to_string();
        let bar = crate::color::ansi(
            " ".repeat($length),
            $fg as usize,
            $bg as usize,
        );
        eprintln!(
            "\n{}",
            [
                bar.clone(),
                crate::color::ansi(
                    if text.is_empty() { String::new() } else { format!("{}", text) },
                    $fg as usize,
                    $bg as usize,
                ),
            ].join("\n")
        );
    }};
    (length=$length:literal, $text:literal, $( $arg:expr ),* $(,)? ) => {{
        $crate::step!(length=$length, format_args!($text, $($arg,)*))
    }};
    () => {{
        $crate::step!("")
    }};
}
#[macro_export]
macro_rules! step_test {
    ($text:literal) => {{
        $crate::step!(length=80, $text)
    }};
    (length=$length:literal, $text:literal, $( $arg:expr ),* $(,)? ) => {{
        $crate::step!(length=$length, format_args!($text, $($arg,)*))
    }};
    (length=$length:literal, $text:expr) => {{
        let (bg, fg) = crate::color::couple(line!() as usize);
        let text = $text.to_string();
        let bar = crate::color::ansi(
            " ".repeat($length),
            fg.into(),
            bg.into(),
        );
        eprintln!(
            "\n{}",
            [
                bar.clone(),
                crate::color::ansi(
                    if text.is_empty() { String::new() } else { format!("{}", text) },
                    bg.into(),
                    fg.into(),
                ),
                bar.clone(),
            ].join("\n")
        );
    }};
    () => {{
        $crate::step!("")
    }};
}

#[macro_export]
macro_rules! admonition {
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, "{}", $message);
    };
    ($color:literal, $title:literal, $message:expr) => {
        $crate::admonition!($color, title=$title, $message);
    };

    ($color:literal, title=$title:literal, $message:expr) => {
        $crate::admonition!($color, title=$title, "{}", $message);
    };
    ($color:literal, title=$title:literal, $format:literal, $($arg:expr),* $(,)?) => {{
        use crate::color;
        eprintln!(
            "\n{}",
            [
                color::ansi(
                    format!("{}:{} {}", crate::function_name!(), line!(), $title),
                    color::invert_bw($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_bw($color).into(),
                )
            ]
            .join(" ")
        );
    }};
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {{
        use crate::color;
        eprintln!(
            "\n{}",
            [
                color::ansi(
                    format!("{}:{}", crate::function_name!(), line!()),
                    color::invert_bw($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_bw($color).into(),
                )
            ]
            .join(" ")
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {
        $crate::admonition!($color, title="WARNING", $format, $($arg),*);
    };
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, title="WARNING", $message);
    };
    ($message:expr) => {
        $crate::warn!(220, $message);
    };
}

#[macro_export]
macro_rules! info {
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {
        $crate::admonition!($color, title="INFO", $format, $($arg),*);
    };
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, title="INFO", $message);
    };
    ($message:expr) => {
        $crate::info!(74, $message);
    };
}
