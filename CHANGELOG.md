# Unreleased

-   Implement generic methods.

    See [`examples/02.rs`](./examples/02.rs) for the full example.

    ```rust
    rcstruct::rcstruct! {
        pub struct GUI {}

        impl {
            pub new() -> Rt<Self> {
                {}
            }

            pub fn window<T>(&self, data: T) -> WindowBuilder<T> {
                WindowBuilder { data }
            }
        }
    }
    ```

# 0.1.0 - 2019-06-07

-   🎉 Initial release.
