# Rust Coding Guidelines

## 1. General Guidelines

- **Code clarity over cleverness**: Prioritize readable and maintainable code over overly optimized or clever solutions.
- **Avoid premature optimization**: Focus on writing clear, correct code first. Only optimize after measuring performance issues.
- **Use `rustfmt`**: Always format your code using `rustfmt`. This ensures consistency and readability across projects.
- **Leverage `clippy`**: Use `clippy` to detect common mistakes and improve your code quality.

## 2. Functions

- **Small, focused functions**: Aim to make functions small and single-purpose.
    - Example: A function should do one thing and do it well.

- **Function signature clarity**: Ensure function signatures are easy to understand. If a function takes too many parameters, consider grouping them into a struct.

- **Avoid deep nesting**: Use early returns (`return` keyword) to avoid deeply nested conditional logic.

- **Use descriptive names**: Functions should clearly describe what they do.
    - Example: `fn calculate_total_price()` is better than `fn calc()`.

- **Use generics and traits when appropriate**: Favor generic code over duplication, and define traits when common behavior needs to be abstracted.

## 3. Error Handling

- **Prefer `Result` and `Option`**: Use Rust's `Result<T, E>` and `Option<T>` types for error handling rather than `panic!`.
    - Example:
      ```rust
      fn parse_number(input: &str) -> Result<i32, ParseIntError> {
          input.parse::<i32>()
      }
      ```

- **Avoid panicking in library code**: Libraries should return errors, not panic. Reserve panics for scenarios where recovery is not possible (e.g., unreachable code).

- **Use `unwrap`/`expect` sparingly**: Prefer graceful error handling over `unwrap`/`expect`, unless you're writing tests or dealing with cases you are 100% sure won't fail.

## 4. Memory Management and Ownership

- **Ownership and borrowing**: Ensure you're familiar with Rust's ownership model and use it effectively:
    - **Move** semantics when transferring ownership.
    - **Borrow** (via references) when you want to avoid taking ownership.

- **Use `&mut` over `Rc<RefCell>`**: When possible, prefer mutable references (`&mut`) over more complex types like `Rc<RefCell>`, which can hide bugs.

- **RAII Pattern**: Take advantage of Rust's RAII (Resource Acquisition Is Initialization) pattern to manage resources. Cleanup will happen automatically when a resource goes out of scope.

## 5. Modules and Visibility

- **Use modules for organization**: Group related functionality into modules. Each module should have a single responsibility.

- **Control visibility**: Use `pub` and `pub(crate)` to control what is exposed from a module. Prefer keeping most things private and exposing only what is necessary.

- **Leverage the `prelude` pattern**: Use a module (e.g., `mod prelude`) to re-export commonly used types and functions across your library.

## 6. Documentation and Comments

- **Document public APIs**: Use Rust's built-in documentation system to document functions, structs, traits, etc.
    - Example:
      ```rust
      /// Calculates the total price of the cart.
      ///
      /// # Arguments
      ///
      /// * `items` - A list of items in the cart.
      fn calculate_total_price(items: &[Item]) -> f64 {
          // ...
      }
      ```

- **Use comments sparingly**: Code should be self-explanatory. Add comments only when necessary to explain why something is done, rather than what is done.

- **Write examples**: For public libraries, provide examples in the documentation for how to use each function or struct.

## 7. Iterators and Collections

- **Use iterators over loops**: Rust's iterators are powerful and expressive. Prefer using them over manual looping.
    - Example:
      ```rust
      let sum: i32 = vec.iter().map(|x| x + 1).sum();
      ```

- **Use collection methods**: Favor using the methods provided by collections (e.g., `push`, `extend`, `drain`) over manually iterating and modifying them.

## 8. Lifetimes

- **Understand lifetimes**: Lifetimes ensure references are valid as long as necessary. Use them when multiple references to the same data need to coexist.

- **Use `'static` only when appropriate**: Avoid `'static` unless you are sure the data will live for the entire program's execution.

## 9. Concurrency and Async Code

- **Prefer threads over unsafe code**: Use Rust's thread-safe features (e.g., `std::sync::Mutex`, `std::sync::Arc`) to avoid data races instead of using `unsafe` code.

- **Use `async/await`**: For asynchronous operations, prefer Rust's `async`/`await` syntax to manage async tasks efficiently.

- **Limit thread spawns**: Avoid spawning too many threads unnecessarily. Use thread pools (e.g., `rayon`, `tokio`) for better performance and resource management.

## 10. Testing and Benchmarks

- **Write unit tests**: Each module or functionality should have unit tests to verify correctness.
    - Example:
      ```rust
      #[cfg(test)]
      mod tests {
          use super::*;
  
          #[test]
          fn test_add() {
              assert_eq!(add(2, 3), 5);
          }
      }
      ```

- **Test edge cases**: Ensure your tests cover edge cases, such as empty collections, null values, and boundary values.

- **Use `cargo bench` for benchmarks**: For performance-sensitive code, write benchmarks using `#[bench]` and run them with `cargo bench`.

---

By following these guidelines, you’ll write clean, efficient, and idiomatic Rust code. Stay consistent and regularly refactor for clarity!

