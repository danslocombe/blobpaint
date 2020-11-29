# Blobpaint

![img](https://github.com/danslocombe/blobpaint-web/raw/master/screenshot.png "Screenshot")
![img](https://github.com/danslocombe/blobpaint-web/raw/master/domblob.gif "Credit to Dominic Englebright")

### Hosted here https://danslocom.be/blobpaint/index.html

---

### Dependencies

- [Rust](https://rustup.rs/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [npm](https://www.npmjs.com/)


### Build and run

In `blobrust` run
```bash
wasm-pack build # To compile the rust sources
cd pkg
npm link        # To tell npm where to source the blobrust package
```

In `site` run
```bash
npm link blobrust # To use the local version of the blobrust package
npm install       # Fetch dependencies
npm run serve     # Launch the local server
```
