# Smart Planner

Smart Planner is a Rust application that helps you organize your tasks, track your progress, and plan your day. It has the following features:

- Todo list: You can create a todo list with low, medium, and high priority tasks. You can mark them as done or delete them as you wish.
- Daily routines: You can set up your daily routines, such as morning exercise, breakfast, study, etc. You can also customize the duration and frequency of each routine.
- Stopwatch: You can start a stopwatch when you are working on a subject. The data (the time and the topic) will be stored either on cloud or a local database, depending on your preference.
- Timer: You can use a timer to set a deadline for a task. When the time is up, you will get a notification.
- Daily plan: You can create a daily plan based on your high priority tasks. The application will use Bing, Bard, ChatGPT, Llama, or Anthropic API to create a plan for you. You can also modify the plan as you like.
- Report: You can generate a report that shows how much you work on each subject, the completion rate of your high priority tasks, and other statistics. You can also get the report as an email.

## Dependencies

To run this project, you will need the following crates:

- `chrono`: for date and time manipulation
- `clap`: for command-line argument parsing
- `reqwest`: for making HTTP requests
- `serde`: for serialization and deserialization
- `sqlite`: for local database operations
- `tokio`: for asynchronous I/O

You can install them by adding them to your `Cargo.toml` file under `[dependencies]`.

## Usage

To run this project, you can use the following commands:

- `cargo run -- todo`: to create, view, edit, or delete your todo list
- `cargo run -- routine`: to create, view, edit, or delete your daily routines
- `cargo run -- stopwatch`: to start, stop, or reset your stopwatch
- `cargo run -- timer`: to set, start, or cancel your timer
- `cargo run -- plan`: to create, view, edit, or delete your daily plan
- `cargo run -- report`: to generate or email your report

You can also use the `-h` or `--help` flag to get more information about each command and its options.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
