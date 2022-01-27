
const listener = Deno.listen({ port: 8000 });
console.log("http://localhost:8000/");

async function handle(conn: Deno.Conn) {
  const requests = Deno.serveHttp(conn);
  for await (const { respondWith } of requests) {
    let count : number = localStorage.getItem("contador")
    count++ 
    localStorage.setItem("contador", count)
    respondWith(new Response("Hola mundo "+count));
  }
}

for await (const conn of listener) {
  handle(conn);
}
