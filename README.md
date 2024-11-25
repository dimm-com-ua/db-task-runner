# Postgres Script Runner

A simple command-line tool written in Rust for executing predefined PostgreSQL scripts, as described in a YAML configuration file. This utility is designed for developers and database administrators who want to automate repetitive database tasks.

---

## Features
- Parse tasks and database connection details from a YAML file.
- Execute SQL scripts for a specific task, identified by a command name.
- Log execution progress and errors using `env_logger`.

---

## Prerequisites
1. **Rust**: Install Rust using [rustup](https://rustup.rs/).
2. **PostgreSQL Database**: Ensure you have access to a PostgreSQL server.
3. **YAML Configuration File**: Prepare a file defining tasks and connections.

---

## Installation
1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>

2. Build the executable
    ```bash
   cargo build --release
3. The binary will be available in `target/release/`

---

## Usage

### Run the tool
    ```bash
    ./postgres-script-runner -f <config-file.yaml> -c <command-name>

### Options
- `-f, --file`: Path to the YAML file containing task definition
- `-c, --command`: Name of the task to execute.

## Configuration
The configuration if provided un a YAML file, describing:
1. Database connection.
2. Tasks with SQL scripts and the connection they use.

### Sample YAML file
```yaml
    tasks:
    - name: create_table
      connection:
          host: "localhost"
          port: 5432
          database: "test_db"
          user: "postgres"
          password: "password"
      script: |
          CREATE TABLE IF NOT EXISTS users (
          id SERIAL PRIMARY KEY,
          name TEXT NOT NULL,
          email TEXT UNIQUE NOT NULL
          );
    - name: insert_data
      connection:
          host: "localhost"
          port: 5432
          database: "test_db"
          user: "postgres"
          password: "password"
          script: |
            INSERT INTO users (name, email) VALUES
            ('Alice', 'alice@example.com'),
            ('Bob', 'bob@example.com');
```

## Example
Run specific task from the YAML file:
```bash
    ./db-task-runner -f config.yaml -c create_table
```

## Output Example
```
    [INFO] Running task [create_table]
    [INFO] Task create_table execution success!
```

## Logging
This tool uses `env_logger` for configurable logging. 
To control the log level, set the `RUST_LOG` environment variable before running the tool. 
For example:
```bash
    RUST_LOG=info ./postgres-script-runner -f config.yaml -c create_table
```

## Error handling
If the task name or YAML file is incorrect, or there are issues connecting to the database, appropriate error messages will be logged.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests