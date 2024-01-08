// const importObject = { imports: { do_loop: (arg) => console.log(arg) } };

const { instance } = await WebAssembly.instantiateStreaming(
  await fetch("target/wasm32-unknown-unknown/debug/rs-wasm.wasm")
);

document.body.textContent = instance.exports.add(1, 2);
