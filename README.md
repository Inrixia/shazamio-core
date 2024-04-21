## shazamio-core

This is a fork of [shazamio-core](https://github.com/shazamio/shazamio-core) for WebAssembly

## Installation

```
npm install shazamio-core
```

## Types

```ts
class DecodedSignature {
	number_samples: number;
	sample_rate_hz: number;
	readonly samplems: number;
	readonly uri: string;
}
```

## Examples

### Node.js

```ts
import { recognizeBytes } from "shazamio-core";
import { readFileSync } from "fs";

const songBytes = readFileSync("./my_song.flac");
const signatures: DecodedSignature[] = recognizeBytes(songBytes);
```

### Web

```ts
import initShazamio, { recognizeBytes } from "shazamio-core/web";
await initShazamio();

// Get bytes from a File in browser from the user
const songBytes = new Uint8Array(await file.arrayBuffer());
const [{ uri, samplems, sample_rate_hz, number_samples }]: DecodedSignature[] = recognizeBytes(songBytes);
```

<br/>

## Methods

### recognizeBytes

Recognizes an audio fingerprint fron song bytes and returns decoded signatures.

```ts
function recognizeBytes(bytes: Uint8Array, offset?: number, seconds?: number): DecodedSignature[];
```

#### Parameters

- `bytes` - Bytes of the song file
- `offset` - When to start sampling from in seconds
- `seconds` - Seconds to sample from offset
