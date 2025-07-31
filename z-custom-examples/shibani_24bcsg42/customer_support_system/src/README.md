#  Customer Support Ticketing System (Rust)

A scalable and feature-rich backend system built using "Rust + Axum", designed to manage customer support tickets across multiple channels. Includes role-based access, filtering, knowledge base, analytics, and real-time-ready architecture.

---

## Features

-  "User Management" with JWT-based roles (Admin, Agent)
-  "Ticket Management" (status, priority, assignment)
-  "Search & Filtering" by status, priority, date, agent, customer
-  "Knowledge Base CRUD" (FAQs and self-help)
-  "Analytics" (basic reports on ticket data)
-  "Pagination" for scalable performance
-  "Swagger API Docs" using Utoipa



##  Tech Stack

- "Rust" + "Axum" – web framework
- "PostgreSQL" – database
- "SeaORM" – ORM
- "JWT" – for role-based auth
- "Utoipa" – OpenAPI/Swagger docs
- "Tokio" – async runtime


 ## Setup Instructions

### 1.  Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Set up PostgreSQL locally or with Docker

### 2. Project Structure

project_root/ ├── src/ 
                 │   
                 ├── main.rs        # Entry point │   
                 ├── api.rs         # All handler functions │   
                 ├── routes.rs      # All route definitions │   
                 ├── auth.rs        # Role-based JWT logic │   
                 ├── error_handle.rs       # Error handling │   
                 ├── app_state.rs          # DB setup (SeaORM) │   
                 ├── docs.rs    # Swagger docs (utoipa) │    
                 │   
                 └── entity/        # SeaORM models 
             ├── Cargo.toml         # Dependencies 
             ├── .env               # Secrets & database config

### 3. Configure .env

 ---env--
DATABASE_URL=postgres://postgres:password@localhost:5432/support_system
JWT_SECRET=your-secret-key

### 4. Run the App

cargo run

Access Swagger UI at:
📍 http://localhost:3000/


## Includes:
Authenticated routes (Admin/Agent)
Ticket filtering & pagination
Knowledge base management
Customer-facing ticket replies