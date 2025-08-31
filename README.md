# etotop_client

## Install fvm and flutter
On Linux DON'T use flutter from snap!<br>
install it manual<br>
https://docs.flutter.dev/install/manual<br>

## install fvm
https://fvm.app/documentation/getting-started/installation<br>

## add paths to bash
export PATH="$HOME/<path to flutter folder>/bin:$PATH" >> ~/.bashrc
export PATH="$HOME/<path to fvm folder>/bin:$PATH" >> ~/.bashrc

## add fvm to dart
dart pub global activate fvm<br>

in project folder run:<br>
fvm use 3.41.7

## On linux ubuntu
sudo apt install -y just curl git unzip xz-utils zip libglu1-mesa libgtk-3-dev libsecret-1-dev lld-18 cmake ninja-build build-essential clang

## Installing flutter_rust_bridge_codegen
cargo install flutter_rust_bridge_codegen --version <version of flutter_rust_bridge from Cargo.toml>

## Getting Started
just gen<br>
just run
