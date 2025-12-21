# Christmas Scavenger Hunt 2025

A Rust + Yew web application for a family Christmas scavenger hunt. Each QR code leads to a page with personalized trivia questions for 3 children.

## Features

- Sequential question flow (one question at a time)
- Review page showing all questions and answers before submission
- Validation: all answers must be correct to proceed
- Hash-based routing for different scavenger hunt pages
- JSON configuration file for easy customization

## Setup

### Prerequisites

- Rust (latest stable version)
- Trunk (`cargo install trunk`)
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

### Development

1. Install dependencies:
   ```bash
   cargo build
   ```

2. Run development server:
   ```bash
   trunk serve
   ```

3. Open browser to `http://127.0.0.1:8080`

### Building for Production

```bash
trunk build --release
```

This creates a `dist/` directory with all the files needed for deployment.

## Configuration

Edit `data.json` to customize the scavenger hunt. The structure is:

```json
{
  "pages": [
    {
      "url": "page1",
      "next_destination": "under the Christmas tree",
      "questions": [
        {
          "child": { "name": "Alice" },
          "question": "What is Alice's favorite color?",
          "correct_answer": "Blue",
          "answers": ["Red", "Blue", "Green", "Yellow"]
        }
      ]
    }
  ]
}
```

## Deployment

The project includes a GitHub Actions workflow (`.github/workflows/deploy.yml`) that automatically builds and deploys to GitHub Pages when you push to the `main` branch.

To deploy manually:

1. Build the project: `trunk build --release`
2. Deploy the `dist/` directory to GitHub Pages

## Usage

1. Generate QR codes pointing to `https://yourusername.github.io/xmas-2025/#page1`, `#page2`, etc.
2. Place QR codes at scavenger hunt locations
3. Children scan QR codes and answer their personalized questions
4. After all questions are answered, they review and submit
5. If all correct, they see the next location; otherwise, they try again

## Project Structure

- `src/models.rs` - Data structures
- `src/config.rs` - JSON loading logic
- `src/router.rs` - Hash-based routing
- `src/components/` - React-like components
  - `page.rs` - Main page orchestrator
  - `question.rs` - Individual question display
  - `review.rs` - Review page
  - `result.rs` - Success/failure display
- `data.json` - Scavenger hunt configuration
