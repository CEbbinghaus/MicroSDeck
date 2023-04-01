// const req = require("./src/usdpl-front/usdpl_front.js")
import { init_usdpl, target_usdpl, init_embedded, call_backend } from "./src/usdpl-front/usdpl_front.js";

const USDPL_PORT = 54321;
// const { call_backend } = req;

console.log("Hello, World!");
(async function () {
	await init_embedded();
	init_usdpl(USDPL_PORT);
	console.log("USDPL started for framework: " + target_usdpl());
	let res = await call_backend("ping", []);
	console.log(res);
})();
