const listener = Deno.listen({ port: 8000 });
console.log("http://localhost:8000/");

async function handle(conn) {
  const requests = Deno.serveHttp(conn);
  for await (const { respondWith } of requests) {
    respondWith(new Response("Hola mundo"));
  }
}

for await (const conn of listener) {
  handle(conn);
}
