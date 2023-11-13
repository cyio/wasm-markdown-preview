# wasm-markdown-preview

这个项目演示了如何使用 Rust 编写 WebAssembly 模块，并通过 JavaScript 调用该模块以实现 Markdown 的实时预览。在这个示例中，我们使用 Rust 的 pulldown-cmark 库来解析 Markdown 文本并将其转换为 HTML。

本地运行
```sh
npx vite serve ./
```