# RustGanizer

RustGanizer is a simple file organizer written in Rust. It helps you organize files on your Windows system by categorizing them into folders based on their types (e.g., music, videos, images, and documents). The program uses a terminal-based user interface powered by the `cursive` crate.

## Features

- Organizes files from your `Downloads` and `Desktop` directories.
- Moves files into appropriate folders such as `Music`, `Videos`, `Pictures`, and `Documents`.
- Provides a simple and interactive user interface.
- Displays statistics about the number of files and folders moved.

## Prerequisites

Before running RustGanizer, ensure you have the following installed:

1. **Rust**: Install Rust using [rustup](https://rustup.rs/).
2. **Cargo**: Cargo is included with Rust and is used to build and run the program.
3. **Windows OS**: This program is designed to work on Windows systems.

## How to Run

Follow these steps to run RustGanizer:

### 1. Clone the Repository

Clone the RustGanizer repository to your local machine:

```bash
git clone https://github.com/your-username/rustganizer.git
cd rustganizer
```

### 2. Build the Project

Build the project using Cargo:

```bash
cargo build
```

### 3. Run the Program
Run the program using Cargo:
```bash
cargo run
```

### 4. Enter Your Windows Username

When prompetd, enter your Windows username. RustGanizer will use this to locate your `Downloads` and `Desktop` directories.

### 5. Organize Files

Select the "Organize" button in the terminal-base IU. RustGanizer will:

- Analyze the files in your `Downloads` and `Desktop` directories.
- Move files into the appropiate folders (`Music`, `Videos`, `Pictures`, `Documents`).

### 6. View Resutls

Once the organization is complete, RustGanizer will display a summary of the files and folders moved.

### 7. Exit the Program

Select the "Close" button to exit the program.

#### Developtment

If you want to modify or contribute to RustGanizer, follow theses stepes:

1. Make changes to the `src` files in the src directory.
2. Rebuild the project using `cargo build`
3. Test your changes by running the program with `cargo run`

#### Troubleshooting
- Error: User not found: Ensure you enter a valid windows username.
- Permission issues: Run the program with administrator privileges if you encounter permissions errors while moving files.

##### License
This project is licensed under the MIT License. See the `LICENSE` file for details.

