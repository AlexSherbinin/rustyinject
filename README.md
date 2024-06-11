# rustyinject

Rustyinject is a compile-time DI library for Rust.

## Overview

Dependency Injection is a design pattern used to implement IoC (Inversion of Control), allowing the creation, storage, and retrieval of dependencies in a flexible and decoupled manner. This provides a container for DI that can:

- Store singleton instances.
- Provide cloned instances of singletons.
- Create dependencies using factory methods.

## Usage

Here is an example of how to use the DI container:

```rust
struct MyService {
    // Some fields
}

struct MyServiceFactory;

impl Factory for MyServiceFactory {
    type Result = MyService;
}

impl<DepsContainer> FactoryBuild<DepsContainer, ()> for MyServiceFactory {
    fn build(&self, _container: &DepsContainer) -> Self::Result {
        MyService {
            // Initialize fields
        }
    }
}

let container = DependencyContainer::default()
    .with_factory(MyServiceFactory);

let my_service: MyService = (&container).inject();
```
## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

