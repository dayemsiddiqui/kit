// src/server.ts
import { createServer } from "http";
import cluster from "node:cluster";
import { availableParallelism } from "node:os";
import * as process from "process";
var readableToString = (readable) => new Promise((resolve, reject) => {
  let data = "";
  readable.on("data", (chunk) => data += chunk);
  readable.on("end", () => resolve(data));
  readable.on("error", (err) => reject(err));
});
var server_default = (render, options) => {
  const _port = typeof options === "number" ? options : options?.port ?? 13714;
  const _useCluster = typeof options === "object" && options?.cluster !== void 0 ? options.cluster : false;
  const log = (message) => {
    console.log(
      _useCluster && !cluster.isPrimary ? `[${cluster.worker?.id ?? "N/A"} / ${cluster.worker?.process?.pid ?? "N/A"}] ${message}` : message
    );
  };
  if (_useCluster && cluster.isPrimary) {
    log("Primary Inertia SSR server process started...");
    for (let i = 0; i < availableParallelism(); i++) {
      cluster.fork();
    }
    return;
  }
  const routes = {
    "/health": async () => ({ status: "OK", timestamp: Date.now() }),
    "/shutdown": () => process.exit(),
    "/render": async (request) => render(JSON.parse(await readableToString(request))),
    "/404": async () => ({ status: "NOT_FOUND", timestamp: Date.now() })
  };
  createServer(async (request, response) => {
    const dispatchRoute = routes[request.url] || routes["/404"];
    try {
      response.writeHead(200, { "Content-Type": "application/json", Server: "Inertia.js SSR" });
      response.write(JSON.stringify(await dispatchRoute(request)));
    } catch (e) {
      console.error(e);
    }
    response.end();
  }).listen(_port, () => log("Inertia SSR server started."));
  log(`Starting SSR server on port ${_port}...`);
};
export {
  server_default as default
};
//# sourceMappingURL=server.esm.js.map
