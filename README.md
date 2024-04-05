# Enigma Machine Simulator 

The Enigma Machine Simulator is a rust implementation of the Enigma machine used during World War II for encrypting and decrypting messages. This implementation aims to accurately represent the rotors, plugboard, and reflector systems of the historical Enigma machine. You can try it out on the github pages [site](https://rbnyng.github.io/enigma_machine/) for this project. 

## Features

- Simulate Enigma Machine: Encrypt and decrypt messages using a simulation of the Enigma machine.
- Customizable Configuration: Set up rotor positions, plugboard pairings, and reflector wirings.
- GUI: A graphical interface built with eframe/egui for easy interaction.

## Installation

To build this from source, you need to have Rust and Cargo installed on your computer. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

### Cloning the Repository

1. First, clone the repository to your local machine:

    ```sh
    git clone https://github.com/rbnyng/enigma_machine
    ```
2. Navigate to the project directory:
    ```sh
    cd enigma_machine
    ```

### Running the Simulation

To compile and run the simulation, use the following command from the project directory:
    ```sh
    cargo run
    ```

Or you can build it into an executable with:
    ```sh
    cargo build --release
    ```
	
### WebAssembly Deployment

To deploy the game as a WebAssembly application and see it in a web browser, use the following commands:

1. Install `trunk`:

    ```sh
    cargo install trunk
    ```

2. Install the required wasm target with:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

#### Web Local Testing

1. Build and serve the game locally on `http://127.0.0.1:8080` with:
    ```sh
    trunk serve
    ```

#### Web Deploy

1. Build the dist with:
    ```sh
    trunk build --release --public-url .
    ```

This generates a `dist` folder as the static html to deploy.

Alternatively, a workflow is included to automatically build and deploy to GitHub Pages.

## Usage

- Set Plugboard Pairs: Enter pairs of characters in the plugboard input field to swap letters before and after rotor encryption/decryption.
- Set Rotor Positions: Specify the starting positions of the rotors to set the initial state.
- Enter Message: Type the message to encrypt or decrypt in the input field.
- Encrypt/Decrypt: Click the `Encode` or `Decode` button to process your message. Encoding and decoding use the same process, so ensure the configuration matches for both operations.

## License

This project is open source and available under the [MIT License](LICENSE).

