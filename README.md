# DCT

Simple Data Consumption Tracker (could have a system information scrapper tool).

Scrape Network Data information and make analytics out of it.
Supposed to be an installable that gives you and endpoint to monitor how much traffic you have on your device.

Things to do:

- [X] Scrape network data
- [ ] Possible CLI interaction?
- [X] Web socket and an endpoint to send stuff.
- [X] Minimal webpage to vizualize info from web server.

## Getting Started

### Prerequisites

- Rust 2021 edition or later

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

4. **Run the Application:**

   ```bash
   cargo run
   ```

## API Endpoints

### WebSocket Endpoint

- **Path:** `/ws`
- **Method:** GET
- **Description:** Establishes a WebSocket connection to receive real-time network metrics updates.
- **Response:** WebSocket connection sending JSON `NetworkMetrics` periodically.
