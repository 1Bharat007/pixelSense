# Engineering Principles

We build PixelSense strictly according to these principles:

1. **Offline First**: All features must function perfectly without an internet connection.
2. **Privacy First**: Zero telemetry, zero analytics, zero data collection. Period.
3. **Native Performance**: Leverage Rust and OS-native APIs to ensure minimal CPU and RAM overhead.
4. **Small Binary Size**: Keep the application lightweight. Avoid unnecessary dependencies.
5. **Accessibility**: The UI must be usable by everyone, including comprehensive keyboard navigation.
6. **Testability**: All business logic must be tested independently of the UI and OS.
7. **Simplicity**: Do not overengineer. Write code that is easy to read and maintain.
8. **Clean Architecture**: Strictly separate Domain, Application Services, and Infrastructure/Platform layers.
9. **Incremental Development**: Deliver small, working features in atomic PRs rather than massive rewrites.
10. **Documentation Before Complexity**: Document the architecture before building complex systems.
11. **Documentation Driven Development**: Write documentation and tests describing the intended behavior *before* writing the implementation.
