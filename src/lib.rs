#[macro_export]
macro_rules! define_arc_wrapped_string_type {
    ( $($element:tt),* $(,)? ) => { $(

        #[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
        pub struct $element(Arc<String>);

        impl $element {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn into_new_type<T: From<Arc<String>>>(self) -> T {
                T::from(self.0)
            }
        }

        impl AsRef<$element> for $element {
            fn as_ref(&self) -> &$element {
                self
            }
        }

        impl From<Arc<String>> for $element {
            fn from(s: Arc<String>) -> Self {
                Self(s)
            }
        }

        impl From<String> for $element {
            fn from(s: String) -> Self {
                Arc::new(s).into()
            }
        }

        impl From<&str> for $element {
            fn from(s: &str) -> Self {
                s.to_string().into()
            }
        }

        impl From<&String> for $element {
            fn from(s: &String) -> Self {
                s.to_string().into()
            }
        }

        impl From<&$element> for $element {
            fn from(s: &$element) -> Self {
                s.0.clone().into()
            }
        }

        impl Into<String> for $element {
            fn into(self) -> String {
                self.0.as_str().to_string()
            }
        }

        impl Into<Arc<String>> for $element {
            fn into(self) -> Arc<String> {
                self.0
            }
        }

        impl Into<String> for &$element {
            fn into(self) -> String {
                self.0.as_str().to_string()
            }
        }

        impl Default for $element {
            fn default() -> Self {
                Self::new("")
            }
        }

        impl $element {
            pub fn new(s: impl Into<$element>) -> Self {
                s.into()
            }
        }
    )* };
}

#[cfg(test)]
mod tests {
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
    }
}
