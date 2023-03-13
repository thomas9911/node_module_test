# Usage

install and build:

```sh
npm install
npm run build
```

open node console:

```js
node_module_test = require("./node_module_test.node");

node_module_test.fibonacci(19);

txt = await node_module_test.readFileAsync("./build.rs");
txt.toString();
```
