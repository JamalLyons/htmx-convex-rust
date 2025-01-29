### Project Overview

**Project Name:** Coding Guessing Game
**Description:** A web application that allows users to take quizzes on programming facts and code snippets, track their scores, and view their progress.

### Tech Stack

- **API Server:** Actix-web (Rust)
- **Styling:** Tailwind CSS
- **JavaScript:** HTMX
- **Template Engine:** Tera-rs
- **Database:** Convex

### Project Structure

1. **Landing Page**
   - Brief introduction to the game
   - Link to the user dashboard

2. **User Dashboard**
   - List of available quizzes
   - Display completion status for each quiz
   - Show the user's master score

3. **Quiz Page**
   - Display quiz questions and multiple-choice answers
   - Submit answers and calculate scores

### To-Do List

#### 1. Set Up the Project

- [x] Initialize a new Rust project using Actix-web.
- [x] Set up Tera as the template engine.
- [x] Install Tailwind CSS for styling.
- [x] Set up HTMX for dynamic content loading.

#### 2. Create the Landing Page

- [ ] Create a `landing.html` template using Tera.
- [ ] Add a brief introduction to the game.
- [ ] Include a link to the user dashboard.

#### 3. User Authentication (Optional)

- [ ] Implement user registration and login functionality (if needed).
- [ ] Store user data (e.g., master score, completed quizzes).

#### 4. Create the Dashboard

- [ ] Create a `dashboard.html` template.
- [ ] Fetch and display the list of quizzes.
- [ ] Show completion status for each quiz.
- [ ] Display the user's master score.

#### 5. Create Quiz Functionality

- [ ] Design a `quiz.html` template for displaying quiz questions.
- [ ] Create a data structure for quizzes (questions, options, correct answers).
- [ ] Implement logic to calculate scores based on user answers.
- [ ] Store completed quizzes and update the master score.

#### 6. API Endpoints

- [ ] Create API endpoints for:
  - Fetching quizzes
  - Submitting quiz answers
  - Retrieving user scores and completion status

#### 7. Styling with Tailwind CSS

- [ ] Apply Tailwind CSS styles to all templates.
- [ ] Ensure the design is responsive and user-friendly.

### 8. Database setup

- [ ] Create a convex project
- [ ] Replace all json data with cloud data
- [ ] Setup and use convex-typegen

#### 8. Testing

- [ ] Test the application for bugs and ensure all features work as expected.
- [ ] Test the scoring system and completion tracking.

#### 9. Prepare for Recording

- [ ] Create a script or outline for your video.
- [ ] Record the project walkthrough, explaining each part of the code.
- [ ] Edit the video and add any necessary graphics or annotations.

### Sample Code Snippets

Here are some code snippets to get you started:

#### Actix-web Server Setup

```rust
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(landing))
            .route("/dashboard", web::get().to(dashboard))
            .route("/quiz/{id}", web::get().to(quiz))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn landing() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Coding Guessing Game!")
}

async fn dashboard() -> HttpResponse {
    HttpResponse::Ok().body("User Dashboard")
}

async fn quiz(web::Path(id): web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Quiz ID: {}", id))
}
```

#### Tera Template Example (landing.html)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="/path/to/tailwind.css" rel="stylesheet">
    <title>Coding Guessing Game</title>
</head>
<body class="bg-gray-100">
    <div class="container mx-auto p-4">
        <h1 class="text-2xl font-bold">Welcome to the Coding Guessing Game!</h1>
        <p class="mt-2">Test your programming knowledge with our quizzes.</p>
        <a href="/dashboard" class="mt-4 inline-block bg-blue-500 text-white p-2 rounded">Go to Dashboard</a>
    </div>
</body>
</html>
```
