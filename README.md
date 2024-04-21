## shazamio-core

This is a fork of [shazamio-core](https://github.com/shazamio/shazamio-core) for WebAssembly

## Installation

```
npm install shazamio-core
```

## Types

```ts
interface Signature {
	samplems: number; // ms of audio sampled for this signature
	uri: string; // encoded sample data
}
```

## Usage

### Node.js

```ts
import { Recognizer } from "shazamio-core";
import { readFileSync } from "fs";

const songBytes = readFileSync("./my_song.flac");
const { samplems, uri }: Signature = Recognizer.recognizeBytes(songBytes, 0);
```

### Web

```ts
import initShazamio, { Recognizer } from "shazamio-core/web";
await initShazamio();

// Get bytes from a File in browser from the user
const songBytes = new Uint8Array(await file.arrayBuffer());
const { uri, samplems } = Recognizer.recognizeBytes(songBytes, 0);
```
