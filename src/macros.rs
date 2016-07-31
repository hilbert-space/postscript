macro_rules! deref {
    (@itemize $($one:item)*) => ($($one)*);
    ($name:ident::$field:tt => $target:ty) => (deref! {
        @itemize

        impl ::std::ops::Deref for $name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
    ($name:ident<$life:tt>::$field:tt => $target:ty) => (deref! {
        @itemize

        impl<$life> ::std::ops::Deref for $name<$life> {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<$life> ::std::ops::DerefMut for $name<$life> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
}

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

macro_rules! read_value(
    ($tape:expr) => (try!(::Value::read($tape)));
    ($tape:expr, $kind:ty) => (try!(<$kind as $crate::Value>::read($tape)));
);

macro_rules! read_walue(
    ($tape:expr, $parameter:expr) => (try!(::Walue::read($tape, $parameter)));
    ($tape:expr, $parameter:expr, $kind:ty) => (
        try!(<$kind as $crate::Walue<_>>::read($tape, $parameter))
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($kind),)* } }
        table! { @implement pub $structure { $($field,)* } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)* }
    );
    (@implement pub $structure:ident {
        $($field:ident,)*
    }) => (
        impl $crate::Value for $structure {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $(::std::mem::forget(::std::mem::replace(&mut table.$field, read_value!(tape)));)+
                Ok(table)
            }
        }
    );
}
