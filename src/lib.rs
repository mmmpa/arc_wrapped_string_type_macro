#[macro_use]
extern crate lazy_static;

pub mod prelude {
    use std::sync::Arc;

    lazy_static! {
        pub static ref EMPTY: Arc<String> = Arc::new("".to_string());
    }
}

#[macro_export]
macro_rules! define_arc_wrapped_string_type {
    ( $($element:tt),* $(,)? ) => { $(

        #[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
        pub struct $element(Arc<String>);

        impl $element {
            pub fn new(s: impl ToString) -> Self {
                let s = s.to_string();
                if s.is_empty() {
                    Self::empty()
                } else {
                   Self(Arc::new(s))
                }
            }

            pub fn empty() -> Self {
                Default::default()
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn untype(self) -> Arc<String> {
                self.0
            }

            pub fn into_new_type<T: From<Arc<String>>>(self) -> T {
                T::from(self.0)
            }
        }

        impl Default for $element {
            fn default() -> Self {
              Self(EMPTY.clone())
            }
        }

        impl AsRef<$element> for $element {
            fn as_ref(&self) -> &$element {
                self
            }
        }

        impl AsRef<str> for $element {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        // From

        impl<T: ToString> From<T> for $element {
            fn from(s: T) -> Self {
                Self::new(s)
            }
        }

        // Into

        impl Into<String> for $element {
            fn into(self) -> String {
                self.0.as_str().to_string()
            }
        }

        impl Into<String> for &$element {
            fn into(self) -> String {
                self.0.as_str().to_string()
            }
        }
    )* };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::Arc;

    define_arc_wrapped_string_type!(TestType, OtherType,);

    #[test]
    fn it_works() {
        let t: TestType = "this is new type".into();
        assert_eq!("this is new type", t.as_str());

        let s: String = t.into();
        assert_eq!("this is new type", s);

        let x: TestType = Default::default();
        let y: TestType = Default::default();
        assert_eq!(x, y);

        let m: TestType = "new type".into();
        let n: OtherType = m.into_new_type();
        assert_eq!("new type", n.as_str());

        let mut h = HashMap::new();
        let k: TestType = "new type".into();
        h.insert(k, 123);

        let t1: TestType = Default::default();
        let t2: TestType = Default::default();
        let t3: TestType = TestType::new("");

        assert_eq!(t1, t2);
        assert_eq!(t2, t3);
    }

    #[test]
    fn test_from_types() {
        TestType::from(11);
        TestType::from("11");
        TestType::from("11".to_string());
    }
}
