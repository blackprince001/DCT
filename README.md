# DCT

Simple Data Consumption Tracker (could have a system information scrapper tool).

Scrape Network Data information and make analytics out of it.
Supposed to be an installable that gives you and endpoint to monitor how much traffic you have on your device.

Things to do:

- [X] Scrape network data
- [ ] Possible CLI interaction?
- [X] Web socket and an endpoint to send stuff.
- [ ] Minimal webpage to vizualize info from web server.

## Directory Structure

```bash
└── blackprince001-DCT/
    ├── .env.sample
    ├── .github/
    │   └── workflows/
    │       ├── release.yml
    │       ├── cross.yml
    │       └── build.yml
    ├── Cargo.toml
    ├── metrics.db
    ├── .cargo/
    │   └── config.toml
    ├── migrations/
    │   └── 20241117234314_initial_tables.sql
    ├── README.md
    └── src/
        ├── main.rs
        ├── network/
        │   ├── db.rs
        │   ├── types.rs
        │   ├── mod.rs
        │   └── net.rs
        ├── lib.rs
        └── server/
            ├── http.rs
            └── mod.rs
```

## Getting Started

### Prerequisites

- Rust 2021 edition or later
- SQLite database
- `metrics.db` file (automatically created if not present)

### Installation and Setup

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-repo/blackprince001-DCT.git
   ```

2. **Navigate to the Project Directory:**

   ```bash
   cd blackprince001-DCT
   ```

3. **Install Dependencies:**

   ```bash
   cargo build
   ```

4. **Set Up Environment Variables:**
   - Optionally, create a `.env` file based on the `.env.sample` provided.
5. **Run the Application:**

   ```bash
   cargo run
   ```

## API Endpoints

### Metrics Endpoint

- **Path:** `/metrics`
- **Method:** GET
- **Description:** Retrieves the current network metrics.
- **Response:** JSON containing `NetworkMetrics` struct.

### Hourly Metrics Endpoint

- **Path:** `/metrics/hourly`
- **Method:** GET
- **Description:** Retrieves the hourly network metrics.
- **Response:** JSON containing a list of `HourlySample` structs.

### WebSocket Endpoint

- **Path:** `/ws`
- **Method:** GET
- **Description:** Establishes a WebSocket connection to receive real-time network metrics updates.
- **Response:** WebSocket connection sending JSON `NetworkMetrics` periodically.

## Database Setup

The project uses a SQLite database to store network metrics.

- The database is initialized and managed by the `SqliteStorage` module in `src/network/db.rs`.
- The migration script is located in `migrations/20241117234314_initial_tables.sql`.

## Code Structure

### Modules and Files

- **src/main.rs:** The main entry point of the application.
- **src/network:** Contains modules for network data scraping and database operations.
  - **db.rs:** Defines the `SqliteStorage` trait and implementation.
  - **types.rs:** Defines data structures for network samples and metrics.
  - **net.rs:** Handles the network data scraping and updating of metrics.
- **src/server:** Contains modules for the web server and API endpoints.
  - **http.rs:** Defines the HTTP router and handlers for API endpoints.
  - **mod.rs:** Module declaration for the server.

## Continuous Integration and Deployment

### GitHub Actions

- The project uses GitHub Actions for building, testing, and deploying the application.
- There are three workflows:
  - **release.yml:** Automates the release process when a new tag is pushed.
  - **cross.yml:** Cross-compiles the application for different targets.
  - **build.yml:** Builds and tests the application on push or pull requests.

## Contributions

- **Fork the Repository:** Create a fork of the repository to make changes.
- **Create a New Branch:** Create a new branch for your changes.
- **Submit a Pull Request:** Submit a pull request with a clear description of the changes.
- **Code Formatting:** Use Rust formatting tools to maintain consistent code style.
- **Testing:** Ensure all changes are accompanied by relevant tests.
