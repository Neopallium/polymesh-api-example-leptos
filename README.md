# Polymesh API Example - Leptos

A [Leptos](https://github.com/leptos-rs/leptos) web application demonstrating integration with Polkadot browser extensions to interact with the Polymesh blockchain. This example shows how to connect to wallet extensions, retrieve accounts, and sign payloads using a Rust/WebAssembly frontend.

## Features

- 🦀 **Rust + WebAssembly** - Built with Leptos for a reactive, type-safe frontend
- 🔌 **Polkadot Extension Integration** - Connect to browser wallets (Polkadot.js, SubWallet, etc.)
- 📝 **Transaction Signing** - Sign payloads using connected accounts
- 🎨 **Modern UI** - Component-based architecture with navigation and state management

## Prerequisites

### Rust Toolchain
1. Install Rust nightly:
   ```bash
   rustup toolchain install nightly --allow-downgrade
   ```

2. Add WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Trunk (the build tool for Rust/WASM apps):
   ```bash
   cargo install trunk
   ```

### Node.js & Yarn
This project uses Vite to bundle JavaScript dependencies (specifically `@polkadot/extension-dapp`).

1. Install Node.js (v18 or later recommended)
2. Yarn is already configured in this project via `packageManager` field

## Project Structure

```
polymesh-api-example-leptos/
├── src/
│   ├── app.rs              # Main app component
│   ├── web3.rs             # Rust bindings for JS web3 functions
│   ├── components/         # Reusable UI components
│   ├── pages/              # Page components and routing
│   └── providers/          # State management
├── js/
│   └── web3.js             # JavaScript interop layer
├── dist/                   # Built JavaScript (generated)
│   └── js/
│       └── web3.js         # Bundled JS with dependencies
├── style/
│   └── main.scss           # Styles
├── vite.config.js          # Vite bundler configuration
└── index.html              # HTML template
```

## Building and Running

### Development Workflow

#### Option 1: Two-Terminal Setup (Recommended for JS Development)

**Terminal 1** - Watch and rebuild JavaScript when it changes:
```bash
yarn install      # First time only
yarn dev:js       # Runs Vite in watch mode
```

**Terminal 2** - Run the Leptos development server:
```bash
trunk serve --open
```

The app will be available at `http://localhost:8080` (or the port Trunk specifies).

#### Option 2: Single Build

If you're not actively modifying JavaScript code:

```bash
yarn install      # First time only
yarn build:js     # Bundle JavaScript dependencies
trunk serve --open
```

### Production Build

```bash
# 1. Bundle JavaScript
yarn build:js

# 2. Build the Rust/WASM app
trunk build --release
```

The output will be in the `dist/` directory, ready to serve statically.

## How It Works

### JavaScript Integration

This project demonstrates how to use external npm packages in a Rust/WASM application:

1. **JavaScript Layer** (`js/web3.js`) - Imports and wraps `@polkadot/extension-dapp` functions
2. **Vite Bundling** - Vite bundles the JS and its dependencies into `dist/js/web3.js`
3. **Rust Bindings** (`src/web3.rs`) - Uses `wasm-bindgen` to call the bundled JavaScript
4. **Trunk Integration** - Copies the `dist` folder to the final build output

### Key Files

- **`vite.config.js`** - Configures Vite to bundle `js/web3.js` and its npm dependencies
- **`src/web3.rs`** - Rust FFI declarations for JavaScript functions using `#[wasm_bindgen]`
- **`package.json`** - Defines npm dependencies and build scripts
- **`index.html`** - Includes `<link data-trunk rel="copy-dir" href="dist" />` to copy bundled JS

## Browser Extension Required

To use this application, you'll need a Polkadot-compatible browser extension:

- [Polkadot.js Extension](https://polkadot.js.org/extension/)
- [SubWallet](https://subwallet.app/)
- [Talisman](https://www.talisman.xyz/)
- Other compatible extensions

## Troubleshooting

### JavaScript bundle not found
Make sure you've run `yarn build:js` or `yarn dev:js` before running `trunk serve`.

### Extension not detected
Ensure you have a Polkadot-compatible browser extension installed and that you've granted permissions to the site.

### Wasm-bindgen version mismatch
This project pins `wasm-bindgen = "=0.2.105"` for compatibility. If you see version errors, ensure your `wasm-bindgen-cli` matches:
```bash
cargo install wasm-bindgen-cli --version 0.2.105
```

## License

This project is provided as-is for educational purposes.
