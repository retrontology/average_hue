# average_hue

## Description
A Rust program written to calculate the average colour(s) of an image and use them to set the colour of Philips Hue lights. It's currently very rough and needs to be compiled to change settings.

## Rust Requirements
- text_io = "0.1.10"
- hueclient = "0.4.1"
- image = "0.24.1"
- kmeans_colors = "0.5.0"
- palette = "0.6.0"
- uuid = { version = "0.8.2", features = ["v4"] }
- rand = "0.8.5"

## Building
Simply build with cargo:
```
cargo b
```

## Setup
The first time you run the program you will need to pair it with the Hue bridge. The program will automatically search for the bridge and prompt you to press the button when it's ready. It will save the bridge info to file so subsequent runs do not need to go through the pairing process

## Usage
```
average_hue <hue group> <image file>
```