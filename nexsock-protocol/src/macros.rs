#[macro_export]
macro_rules! service_command {
    {
        $(#[$attr:meta])*
        $vis:vis struct $command:ident<_, $output:ty> = $item:ident
    } => {
        // #[cfg_attr(feature = "savefile", derive(::savefile::prelude::Savefile))]
        $(#[$attr])*
        $vis struct $command;

        impl ::std::default::Default for $command {
            fn default() -> Self {
                Self
            }
        }

        impl $command {
            $vis fn new() -> Self {
                ::std::default::Default::default()
            }
        }

        impl $crate::traits::ServiceCommand for $command {
            type Input = ();
            type Output = $output;

            const COMMAND: $crate::commands::Command = $crate::commands::Command::$item;

            fn into_payload(self) -> Self::Input {}
        }
    };

    {
        $vis:vis struct $command:ident<$input:ident, $output:ty> = $item:ident
    } => {
        // #[cfg_attr(feature = "savefile", derive(::savefile::prelude::Savefile))]
        $vis struct $command($input);

        impl $command {
            $vis fn new(input: impl Into<$input>) -> Self {
                Self(input.into())
            }
        }

        impl $crate::traits::ServiceCommand for $command {
            type Input = $input;
            type Output = $output;

            const COMMAND: $crate::commands::Command = $crate::commands::Command::$item;

            fn into_payload(self) -> Self::Input {
                self.0
            }
        }

        impl From<$command> for <$command as $crate::traits::ServiceCommand>::Input {
            fn from(command: $command) -> Self {
                use $crate::traits::ServiceCommand;
                command.into_payload()
            }
        }

        impl From<$input> for $command {
            fn from(input: $input) -> Self {
                Self::new(input)
            }
        }
    };

    {
        $vis:vis struct $command:ident<$input:ty, $output:ty> = $item:ident {
            $($field_vis:vis $field:ident: $field_ty:ty),* $(,)?
        }
    } => {
        // #[cfg_attr(feature = "savefile", derive(::savefile::prelude::Savefile))]
        $vis struct $command {
            $($field_vis $field: $field_ty),*
        }

        impl $command {
            $vis fn new($($field: impl Into<$field_ty>),*) -> Self {
                $(let $field = $field.into();)*

                Self {
                    $($field),*
                }
            }
        }

        impl From<$command> for <$command as $crate::traits::ServiceCommand>::Input {
            fn from(command: $command) -> Self {
                use $crate::traits::ServiceCommand;
                command.into_payload()
            }
        }

        impl $crate::traits::ServiceCommand for $command {
            type Input = $input;
            type Output = $output;

            const COMMAND: $crate::commands::Command = $crate::commands::Command::$item;

            fn into_payload(self) -> Self::Input {
                Self::Input {
                    $($field: self.$field),*
                }
            }
        }

        impl From<$input> for $command {
            fn from(input: $input) -> Self {
                Self {
                    $($field: input.$field),*
                }
            }
        }
    };
}
