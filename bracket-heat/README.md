# Bracket-Heat

An in-development RF heat-mapping system.

## Requirements

* This requires stable Rust 1.88 or newer. Rocket 0.5 no longer needs nightly.
* Set the `GOOGLE_MAPS_API_KEY` environment variable to your Google Maps JavaScript API key before starting the server:
  ~~~bash
  export GOOGLE_MAPS_API_KEY="your-google-maps-api-key"
  ~~~
  The server will refuse to start with a clear error if it isn't set.

## Setup

* Edit `isp.ron` in `resources` to match your WISP.

