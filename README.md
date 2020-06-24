[![CircleCI](https://circleci.com/gh/mmmpa/wrapped_string_type_macro.svg?style=shield)](https://circleci.com/gh/mmmpa/wrapped_string_type_macro)

# wrapped_string_type_macro

```rust
use std::sync::Arc;
use serde::{Deserialize, Serialize};

define_arc_wrapped_string_type!(
    TestType
);

let t: TestType = "this is new type".into();
```