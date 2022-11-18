import { Context } from "https://edge.netlify.com";
import wasmCode from './rust_rewriter/target/wasm32-wasi/debug/rust_rewriter.wasm';

const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const greet = wasmInstance.exports.greet as CallableFunction;

export default async (request: Request, context: Context) => {
  const url = new URL(request.url);

  // // Look for the query parameter, and return if we don't find it
  // if (url.searchParams.get("method") !== "transform") {
  //   return;
  // }

  const response = await context.next();
  const text = await response.text();
  const greeting = greet("Stan");
  return new Response(greeting, response);
};
