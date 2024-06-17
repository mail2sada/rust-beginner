## Writing and Running Your First Rust Program

### Introduction

In this guide, you'll learn how to write and run your first Rust program. We'll cover setting up your development environment, creating a new project, writing the program, and running it. This tutorial assumes you have already set up Rust and Visual Studio Code (VS Code) on your system. If not, please refer to the previous guide on setting up Rust and VS Code.

### Step-by-Step Guide

#### 1. Open Visual Studio Code

1. **Launch Visual Studio Code** on your computer.
2. **Open the integrated terminal** by selecting `Terminal` > `New Terminal` from the menu, or by pressing `` Ctrl+` ``.

#### 2. Create a New Rust Project

Rust projects are managed by Cargo, Rust’s package manager and build system.

1. **Navigate to your desired directory** where you want to create the project using the terminal. For example:

    ```sh
    cd path/to/your/projects/directory
    ```

2. **Create a new Rust project** using Cargo:

    ```sh
    cargo new hello_world
    ```

3. **Navigate into the project directory**:

    ```sh
    cd hello_world
    ```

4. **Open the project in Visual Studio Code**:

    ```sh
    code .
    ```

#### 3. Write Your First Rust Program

1. **Open the `main.rs` file** located in the `src` directory. By default, Cargo creates a simple "Hello, world!" program for you. The file should look like this:

    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

2. **Edit the `main.rs` file** to customize the message if you want. For example:

    ```rust
    fn main() {
        println!("Hello, Rust!");
    }
    ```

3. **Save the file** by pressing `Ctrl+S` or selecting `File` > `Save` from the menu.

#### 4. Build and Run Your Program

1. **Open the integrated terminal** in Visual Studio Code if it isn't open already.
2. **Build and run the program** using Cargo by typing the following command in the terminal:

    ```sh
    cargo run
    ```

3. **View the output** in the terminal. You should see something like:

    ```sh
    Compiling hello_world v0.1.0 (/path/to/hello_world)
        Finished dev [unoptimized + debuginfo] target(s) in 0.50s
         Running `target/debug/hello_world`
    Hello, Rust!
    ```

### Diagram: Rust Project Structure

![Rust Project Structure](https://via.placeholder.com/800x400?text=Rust+Project+Structure)

### Explanation of the Rust Project Structure

- **Cargo.toml**: This file contains metadata about your project, including dependencies and package information.
- **src**: This directory contains your Rust source code.
  - **main.rs**: The main entry point of your application. By default, it contains a simple "Hello, world!" program.

### Final Notes

Congratulations! You've written and run your first Rust program. This simple exercise demonstrates how easy it is to get started with Rust using Cargo and Visual Studio Code. As you continue learning Rust, you'll appreciate its powerful features and safety guarantees, making it an excellent choice for systems programming and beyond.

### Next Steps

- Explore the [Rust documentation](https://doc.rust-lang.org/book/) to learn more about the language.
- Experiment with adding more functionality to your `main.rs` file.
- Learn about Rust's package manager, Cargo, to manage dependencies and build processes effectively.

By following this guide, you should now have a solid foundation for writing and running Rust programs. Happy coding!