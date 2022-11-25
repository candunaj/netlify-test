import { Context } from "https://edge.netlify.com";
// import wasmCode from './rust_rewriter/target/wasm32-wasi/debug/rust_rewriter.wasm';


export default async (request: Request, context: Context) => {
  const url = new URL(request.url);
  // const wasmCode = await Deno.readFile("rust_rewriter.wasm");
  // const wasmModule = new WebAssembly.Module(wasmCode);
  // const importObject = {
  // };
  // const wasmInstance = new WebAssembly.Instance(wasmModule,importObject);
  // const greet = wasmInstance.exports.greet as CallableFunction;
  // const { instance, module } = await WebAssembly.instantiateStreaming(
  //   fetch("https://wpt.live/wasm/incrementer.wasm")
  // );
  // const increment = instance.exports.increment as (input: number) => number;

  const response = await context.next();
  const text = await response.text();
  // const file = await Deno.readFile("test.txt");
  // const greeting = greet("Stan");
  return new Response('<html><head></head><body>STAN</body></html>', response);
};
