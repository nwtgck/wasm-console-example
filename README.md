# wasm-console-example
Simple example of [wasm-console](https://github.com/nwtgck/wasm-console)

## Actual Web page
Open the link below and see the console.  
<https://wasm-console-example.netlify.app/>

## Structure

- [src/lib.rs](src/lib.rs): Rust using wasm-console
- [index.js](index.js): Call WebAssembly from JavaScript


## Run locally

```bash
cd <this repo>
npm ci
npm run serve
```

Then, open <http://localhost:808> on your browser.
