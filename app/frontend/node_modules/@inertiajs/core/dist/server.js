"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/server.ts
var server_exports = {};
__export(server_exports, {
  default: () => server_default
});
module.exports = __toCommonJS(server_exports);
var import_http = require("http");
var import_node_cluster = __toESM(require("node:cluster"), 1);
var import_node_os = require("node:os");
var process = __toESM(require("process"), 1);
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
      _useCluster && !import_node_cluster.default.isPrimary ? `[${import_node_cluster.default.worker?.id ?? "N/A"} / ${import_node_cluster.default.worker?.process?.pid ?? "N/A"}] ${message}` : message
    );
  };
  if (_useCluster && import_node_cluster.default.isPrimary) {
    log("Primary Inertia SSR server process started...");
    for (let i = 0; i < (0, import_node_os.availableParallelism)(); i++) {
      import_node_cluster.default.fork();
    }
    return;
  }
  const routes = {
    "/health": async () => ({ status: "OK", timestamp: Date.now() }),
    "/shutdown": () => process.exit(),
    "/render": async (request) => render(JSON.parse(await readableToString(request))),
    "/404": async () => ({ status: "NOT_FOUND", timestamp: Date.now() })
  };
  (0, import_http.createServer)(async (request, response) => {
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
//# sourceMappingURL=server.js.map
