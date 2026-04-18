# Intelligent Nutrition & Health Management

Intelligent Nutrition & Health Management is a robust, high-performance backend application built with Rust. It serves as the core engine for a health and nutrition tracking ecosystem, featuring personalized meal planning, AI-driven recommendations, and comprehensive user health profile management.

## 🚀 Features

### 🔐 Authentication & Security
- **JWT Authentication**: Secure stateless authentication using JSON Web Tokens.
- **Secure Password Hashing**: Uses `bcrypt` for industry-standard password security.
- **Protected Routes**: Middleware-enforced authorization for sensitive user data.

### 👤 User Health Profiles
- **Profile Management**: Track and update vital statistics (Age, Gender, Activity Level).
- **Goal Setting**: Set and monitor personal health and fitness goals.
- **Medical Tracking**: Manage medical conditions to ensure nutritional recommendations are safe and appropriate.

### 🍽️ Meal & Nutrition Management
- **Dynamic Meal Database**: Comprehensive collection of meals with automatic seeding capabilities.
- **Food Preferences**: Whitelist or blacklist specific foods to personalize the experience.
- **Intelligent Recommendations**: Generate personalized meal suggestions based on health profiles and preferences.

### 📅 Advanced Scheduling
- **Daily Activity Planning**: Create and manage general schedules for daily tasks.
- **Specialized "Eat Schedules"**: Dedicated meal planning with views for today's plan, past history, and future ranges.
- **Shopping Integration**: Manage shopping items directly linked to your schedules.

### 🤖 AI Chatbot Assistant
- **Context-Aware Chat**: Integrated AI (OpenAI) to answer health, fitness, and nutrition questions.
- **History Persistence**: Saves chat history so users can track progress and previous inquiries.

### 🔔 Notification System
- Stay updated with real-time notifications for scheduled events and important health reminders.

---

## 🛠 Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/) (Multi-threaded async runtime)
- **Database**: [MongoDB](https://www.mongodb.com/)
- **Authentication**: `jsonwebtoken`
- **Utility**: `serde` (Serialization), `chrono` (Date/Time), `dotenv` (Env management)
- **AI Integration**: OpenAI API

---

## ⚙️ Setup & Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- [MongoDB](https://www.mongodb.com/try/download/community) account or local instance

### Environment Variables
Create a `.env` file in the root directory and configure the following:

```env
OPENAI_API_KEY=your_openai_api_key
JWT_SECRET=your_secret_key
PEXELS_API_KEY=your_pexels_key
```

*Note: MongoDB URI is currently configured in `src/db/mongo.rs`. For production, it is recommended to move this to the `.env` file.*

### Running the Project

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd backend
   ```

2. **Run the application**:
   ```bash
   cargo run
   ```
   The server will start at `http://localhost:3000`.

---

## 📂 Project Structure

- `src/controller`: API route handlers and request logic.
- `src/models`: Data structures and MongoDB collection schemas.
- `src/services`: Core business logic and database interactions.
- `src/middleware`: Custom authentication and CORS layers.
- `src/utils`: Helper functions for AI, seeding, and external APIs.
- `src/db`: Database connection and initialization.

---

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.
