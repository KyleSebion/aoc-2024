wasm-pack build --target web
del pkg\.gitignore
start http://localhost:8000/
wsl python3 -m http.server
