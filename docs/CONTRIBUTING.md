# Table of Contents
- [Getting Started 🛸](#getting-started-)
- [Code Structure and Format 🍦](#code-structure-and-format-)
- [Request Features and Report Bugs 🧸](#bug-reports-and-features-)

If you are not interested in writing code, please refer to **Request Features and Report Bugs**!

## Getting Started 🛸
The application is primarily broken into two primary parts. First, **Rust** is used as the back-end (think doing the decryption, modifying files, etc). Second, **Tauri** is used to bridge the front-end. You can find the back-end in `src-backend` and the front-end in `src`. Now, let's get started! 

**Please note, that these instructions only apply if you want to contribute towards development.** Please head to the [Releases](https://github.com/kleineluka/burial/releases) section if you are simply planning to use the program.

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [NodeJS](https://nodejs.org/en/download/package-manager) installed on your system.
2. Clone the Github repository with `git clone https://github.com/kleineluka/burial` in your terminal.
3. Navigate to the cloned repistory (ex. `cd burial`) and install all Node packages with `npm install`.
4. Navigate to the back-end directory (ex. `cd src-backend`) and install all Rust crates with `cargo build`.
5. Now, from the main directory (ex. `cd ..`) you can run `npm run tauri dev` to run Burial!

## Code Structure and Format 🍦
Upon opening the front-end (`src`), you will see a few folders alongside `index.html` and `settings.html`. `assets` contains code shared throughout the whole project and in the two html files in the main page, `data` contains json structures that are read from the front-end from certain features (ex. templates, sifting), and `sub` contains all of the sub-menus provided in the program (ex. Resources, Reversing, and so on). The structure of each sub-menu is similar to the main `src` structure without the `data` and `sub` directories. If you want to, for example, edit the "Encryption" page - you would do the following:
1. Find the HTML page in `src/sub/resources/encryption.html`.
2. Find the Javascript and CSS in `src/sub/resources/assets/js/encryption.js` and `src/sub/resources/assets/css/encryption.css`.

Upon opening the back-end (`src-backend`), you will want to then navigate to `src`. You will then see `main.rs`, `settings.rs`, and a few folders. Folders like `config` and `utils` are used throughout the project and are mostly just public helper functions. Folders like `resources`, `modmanager`, and `reversing` all correspond to their sub-menus in the front-end (for example the code in `src-backend/src/resources` will be called by the front-end in `src/sub/resources`). If you want to, for example, edit the code behind the "Encryption" page - you would do the following:
1. Find the corresponding Tauri commands in `src-backend/src/resources/encryption.rs`.
2. Find the helper functions used in `src-backend/src/utils/cypher.rs`.

Please note that all Rust is typically in **snake case** and all Javascript is in **camel case**. Tauri automatically translates between the two. Please make sure that your code is **commented** as well to make it easier for future contributors!

## Bug Reports and Features 🧸
Please just head over to the Github's [Issues Page](https://github.com/kleineluka/burial/issues) and use one of the templates! I'm excited to hear your feedback!
