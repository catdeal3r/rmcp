# rcmp

A 100% rust MCP implementation.

## Usage

Is in early stages.

Run this to start the ollama server (if its not already running)
```bash
ollama serve
```

Run rcmp:
```bash
git clone https://github.com/catdeal3r/rcmp
cd rcmp
cargo run  
```

(If you're on nixos, run `nix develop` before running `cargo run`)
(If you're not on nixos, you need the lib of openssl installed)

## Roadmap
(note this is kinda random as I just finish what I feel is most important first)
(note again, **bold** is most important stuff)

- **Basic chatting** ✅
- **History** ✅
- Tools (can add more if wanted/needed)
  - **FileWrite** ✅
  - **FileRead** ✅
  - **WebSearch** ✅
  - WebFetch
  - **Output** ✅
  - **PathNav**
- **Thinking loops (allowing the ai to use multiple tools in a row to complete a task)** ✅
- Model switching (currently only supports gemma4:e2b)
- **Markdown output** ✅
- **Starter splash screen** ✅
- Spinners and niceties ✅
- Streamed output
- **Model keepalive (leaves the model loaded in ram to reduce processing times)**
- Other source support (e.g. cerebras, openai, etc)
- **Project context**
- History saving
- CLI support (currently is interactive cli)

