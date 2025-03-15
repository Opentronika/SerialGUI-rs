# SerialGUI-Rs

A lightweight serial terminal monitor written in Rust

**SerialGUI-Rs** is a cross-platform graphical serial monitor application that uses the [**serialport-rs**](https://github.com/serialport/serialport-rs) library as its backend and [**eframe**](https://github.com/emilk/egui/tree/master/crates/eframe) for the graphical interface.

## Features

- Real-time serial port monitoring.
- Cross-platform support (Windows, macOS, Linux).
- Intuitive and user-friendly graphical interface.
- Configuration of serial communication parameters (baud rate, parity, etc.).

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your_username/SerialGUI-Rs.git
    ```
2. Navigate to the project directory:
    ```sh
    cd SerialGUI-Rs
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

1. Run the application:
    ```sh
    cargo run --release
    ```
2. Select the serial port and configure the parameters according to your needs.
3. Start monitoring the serial communication.

## Contribution

Thank you for considering contributing to **SerialGUI-Rs**! Here are some basic rules for contributing to the project, following the guidelines of GNU projects:

1. **Install git hooks**
```sh
./setup-hooks.sh
```

2. **Clean and documented code**: Ensure your code is well-documented and follows the project's style conventions.
3. **Clear commits**: Make small, clear commits with descriptive messages.
4. **Pull requests**: Submit your changes through pull requests and make sure your code passes all tests before submitting.
5. **Open discussion**: If you have any ideas or suggestions, feel free to open an issue to discuss it with the community.

For more details, check the GNU Contribution Guide.

## Contributors


<!-- Copy-paste in your Readme.md file -->

<a href="https://github.com/Opentronika/SerialGUI-rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Opentronika/SerialGUI-rs" />
</a>

Made with [contrib.rocks](https://contrib.rocks).



