// server.ts

const server = Bun.serve({
  port: 3000,

  fetch(req) {
    const url = new URL(req.url);

    if (url.pathname === "/") {
      return new Response("Hello from Bun 🚀");
    }

    if (url.pathname === "/api/hello") {
      return Response.json({
        message: "Hello, Izac 👋",
      });
    }

    return new Response("Not Found", { status: 404 });
  },
});

console.log(`Server running at http://localhost:${server.port}`);