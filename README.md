# Christmas Treasure Hunt 2025

A Rust + Yew web application for a family Christmas treasure hunt. From the
start page, each child is asked a multiple-choice trivia question related to
their personal interests. After answering, the next child receives a question.
When all children have answered, a review screen allows retrying or submitting.
On submit, the answers are checked. If incorrect, they must try again. No
feedback is supplied around which question was wrong, encouraging collaborative
conversation. When all answers are correct, the children are presented a
location, which they must explore to find the next "code word". Entering the
code word leads to a new round of questions. This cycle repeats until all
questions are exhausted.

## Features

- Sequential question flow (one question at a time)
- Review page showing all questions and answers before submission
- Validation: all answers must be correct to proceed
- a `print` route that generates "clue cards", to be printed and placed at the
  specified destination.

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

Edit `models/tree.rs` to customize the scavenger hunt.

## Deployment

The project includes a GitHub Actions workflow (`.github/workflows/deploy.yml`)
that automatically builds and deploys to GitHub Pages when you push to the
`main` branch.

To deploy manually:

1. Build the project: `trunk build --release`
2. Deploy the `dist/` directory to GitHub Pages

## Usage

1. Generate "hint cards".
2. Place QR codes at scavenger hunt locations
3. Children scan QR codes and answer their personalized questions
4. After all questions are answered, they review and submit
5. If all correct, they see the next location; otherwise, they try again

## Project Structure

- `src/models/` - Data structures
  - `page.rs` - includes the Page and PageCard data structures
  - `question.rs` - includes the Question data structure
  - `tree.rs` - includes the static declaration of the questions and answers,
     plus accessor functions.
- `src/router.rs` - URL-based routing
- `src/components/` - React-like components
  - `fragments.rs` - HTML generation functions for simple HTML.
  - `page.rs` - Main page orchestrator
  - `print.rs` - A page of "clue cards", suitable for printing on 3.5" x 5" cards
  - `question.rs` - Individual question display
  - `retry.rs` - Retry button resets the page state
  - `review.rs` - Review page for reviewing all answers
  - `success.rs` - Success display with an entry for the next "code word", which
     is used as the next URL.