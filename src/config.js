import "./app.css";
import Config from "./Config.svelte";

const app = new Config({
  target: document.getElementById("app"),
});

export default app;
