## shazamio-core

This is a fork of [shazamio-core](https://github.com/shazamio/shazamio-core) for WebAssembly

## Installation

```
npm install shazamio-core
```

## Usage

```ts
import { Recognizer } from "shazamio-core";
import { readFileSync } from "fs";

const songFile = readFileSync("./my_song.flac");
const { samplems, uri } = Recognizer.recognizeBytes(songFile, 0);

console.log(samplesms, uri);
```
