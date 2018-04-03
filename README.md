##  Rapid Web Application Scaffold

This repository provides a ready-to-deploy web application foundation built with Rust and Vue.js.

**Prerequisites:**

- PostgreSQL database named 'webapp'

**Development Setup:**

1. Clone the repository:
   ```bash
   git clone <repository_url>
   ```

2. Install Diesel CLI:
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

3. Initialize Diesel:
   ```bash
   diesel setup
   ```

4. Start the backend:
   ```bash
   cargo run
   ```

5. Navigate to the frontend directory and install dependencies:
   ```bash
   cd webapp
   npm install
   ```

6. Run the development server:
   ```bash
   npm run dev 
   ```

7. Access the application at `http://localhost:1234/`

**Production Deployment:**

1. Clone the repository and install Diesel CLI (as per development setup).
2. Initialize Diesel (as per development setup).
3. Navigate to the frontend directory and install dependencies:
   ```bash
   cd webapp
   npm install
   ```
4. Build the frontend:
   ```bash
   npm run build
   ```
5. Navigate back to the root directory and start the backend:
   ```bash
   cd ..
   cargo run
   ```
6. Access the application at `http://localhost:8000/`


**Features:**

- User registration and login
- Data browsing (GET/POST)
- Article publishing
- User-centric functionalities with JWT authentication


**Collaboration:**

Contributions are welcome!

**License:**

Apache License 2.0 (See [LICENSE](LICENSE)) 
