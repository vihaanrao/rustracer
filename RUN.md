# How to run the Ray Tracing in Rust project

Follow these steps to set up and run the project on your local machine:

## Prerequisites

- Install Rust programming language (https://www.rust-lang.org/tools/install)
- Install Git (https://git-scm.com/downloads)

## Step 1: Clone the repository

First, clone the project repository from GitHub:

```

git clone https://github.com/yourusername/ray-tracing-in-rust.git

```

Replace `yourusername` with your GitHub username, and make sure the repository URL is correct.

## Step 2: Change to the project directory

Navigate to the project's root directory:

```

cd ray-tracing-in-rust
```


## Step 3: Build the project

Build the project using the following command:



```

cargo build --release
```

This will compile the project with optimizations, and it may take a few minutes to complete.

## Step 4: Run the project

Run the project using the following command:

```

cargo run --release
```

This will execute the compiled binary, generate the output image (output.ppm), and save it in the project's root directory.

## Step 5: View the output

You can view the output image (output.ppm) using an image viewer that supports the PPM format, or you can convert it to a more common format like JPEG or PNG using an image conversion tool.

That's it! You have successfully built and run the Ray Tracing in Rust project.
