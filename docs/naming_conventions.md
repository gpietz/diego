# Rust Naming Conventions Cheat Sheet

## 1. Files and Modules

- **snake_case** for file and module names.
    - Example: `my_module.rs`, `utility_functions.rs`

- **lib.rs** or **main.rs** for entry points in libraries or binaries.

- **mod.rs** is optional for sub-modules, but it's recommended to use **directory-based** modules where possible:
    - Example:
        - Directory structure:
          ```
          src/
            network/
              mod.rs  (optional)
              tcp.rs
              udp.rs
          ```
        - Alternatively:
          ```
          src/
            network/
              tcp.rs
              udp.rs
          ```

## 2. Functions, Variables, and Constants

- **snake_case** for function names and local variables.
    - Example: `calculate_sum()`, `let my_variable = 5;`

- **UPPER_SNAKE_CASE** for constants and static variables.
    - Example: `const MAX_SIZE: usize = 100;`, `static mut GLOBAL_STATE: i32 = 0;`

## 3. Structs, Enums, and Traits

- **CamelCase** for struct, enum, and trait names.
    - Example: `struct MyStruct`, `enum EventType`, `trait MyTrait`

- **PascalCase** for fields in tuples or named fields in structs:
    - Example:
      ```rust
      struct Person {
          FirstName: String,
          LastName: String,
      }
      ```

## 4. Pluralization

- **Singular** when referring to a single entity or type.
    - Example: `struct User`, `fn get_user() -> User`

- **Plural** when referring to a collection or multiple entities.
    - Example: `fn get_users() -> Vec<User>`, `let files = vec![file1, file2];`

- For **types and modules**, use singular when referring to what they represent:
    - Example: `struct Point`, `mod network`

- For **collections**, use plural if the collection contains multiple instances:
    - Example: `mod users`, `mod items`

## 5. Lifetimes and Type Parameters

- **Lowercase single letters** for lifetime parameters.
    - Example: `<'a>`, `<'b>`

- **Uppercase single letters** for generic type parameters.
    - Example: `fn my_func<T>(value: T)`

## 6. Macros

- **snake_case** for macro names.
    - Example: `macro_rules! my_macro`

- Macros should follow similar conventions to functions or variables but may use repetition for collections.
    - Example:
      ```rust
      macro_rules! create_vectors {
          ($($name:ident),*) => {
              $(let $name: Vec<i32> = Vec::new();)*
          };
      }
      ```

---

### Singular vs. Plural Usage

- **Use singular** for:
    - Structs, Enums, Traits, Functions (representing a single action or entity).
    - Example: `struct Car`, `fn get_item() -> Item`

- **Use plural** for:
    - Collections, Modules dealing with multiple entities.
    - Example: `mod cars`, `fn list_items() -> Vec<Item>`

By following these conventions, your Rust code will remain consistent and easier to maintain.

---

