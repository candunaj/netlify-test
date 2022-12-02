import { Context } from "https://edge.netlify.com";
// import wasmCode from './rust_rewriter/target/wasm32-wasi/debug/rust_rewriter.wasm';
// const rust = import("rust_rewriter.wasm");
// import initSync from rust_rewriter.js
import init, { remove_header } from "../rust-rewriter/pkg/rust_rewriter.js";


export default async (request: Request, context: Context) => {
  const url = new URL(request.url);
  await init();
  // const wasmCode = await Deno.readFile("rust_rewriter_bg.wasm");
  // initSync(wasmCode);

  const response = await context.next();
  const text = await response.text();
  // const file = await Deno.readFile("test.txt");
  // date in format %Y-%m-%d %H:%M:%S
  const showUntil = "2022-12-03 00:00:00";
  const newText = remove_header(text, "#simplabsheader", showUntil, true);
  return new Response(newText, response);
};
