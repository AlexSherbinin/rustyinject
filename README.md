# rustyinject

Rustyinject is a compile-time DI library for Rust.

### Overview

Dependency Injection is a design pattern used to implement IoC (Inversion of Control), allowing the creation, storage, and retrieval of dependencies in a flexible and decoupled manner. This provides a container for DI that can:

- Store singleton instances and provide them.
- Provide cloned instances of singletons.
- Create instances using factory methods.

### Usage

Here is an example of how to use the DI container:

```rust
use rustyinject::{DependencyContainer, injector::{factories::ConstructorFactory, Injector}};

struct MyService {
    // Some fields
}

impl ConstructorFactory for MyService {
    type Dependencies<'a> = (); // Specify your dependencies here.

    fn build(dependencies: Self::Dependencies<'_>) -> Self {
        Self {
           // Some fields
        }
    }
}

let container = DependencyContainer::default()
    .with_constructor_factory::<MyService>();

let my_service: MyService = (&container).inject();
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
