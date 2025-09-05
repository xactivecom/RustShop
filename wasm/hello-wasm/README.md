# WASM Setup and Build

## Install NVM, Node and Npm
Check the current version from the site https://nodejs.org.

Install NVM installer:
	% curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
	% source ~/.bashrc
Check possible Node versions:
	% nvm list-remote
Install Node v22:
	% nvm install v22.19.0
Check Node, Npm versions
	% nvm list
	% node --version
	% npm --version

## WASM Setup
Download and run wasm-pack installer:
	% curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


# WASM Project
Follow the procedure: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm

Create a new WASM project
    % cargo new --lib hello-wasm
    % cd hello-wasm

Replace the src/lib.rs file with contents in this project.

Edit the Cargo.toml with these:
- library crate-type = ["cdylib"] for Cargo and Rust linkage
- dependency "wasm-bindgen" = "0.2"

Build the project into a webassembly module for the web
    % wasm-pack build --target web

Create a index.html that invokes the webassembly module.

To
    % wasm-pack build --target bundler

    % mkdir site && cd site
    % npm i ../pkg

Install the webpack v5 dependencies:
    % npm i -D webpack@5 webpack-cli@5 webpack-dev-server@5 copy-webpack-plugin@12
Edit the webpack.config,js as per the project file.
Edit the package.json to add runtime settings.
Create an index.html and index.js file to load webassembly.

Run the web server and browse site at http://localhost:8080:
    % npm run serve
