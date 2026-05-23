import sys
import os
import requests
from pathlib import Path

def get_agent_config(role):
    configs = {
        "architect": {
            "system_prompt": "You are the GNC Architect for Shipyard_GNC. Write clear technical specs in Markdown for RUST implementation. Update docs/spec/ARCH_SPEC.md with new sections. Use equations in LaTeX. Focus on marine guidance, navigation, control. Output architecture that maps directly to Rust structs/traits.",
            "default_file": "docs/spec/ARCH_SPEC.md"
        },
        "coder": {
            "system_prompt": "You are the Rust GNC Coder using qwen2.5-coder:7b. Implement ARCH_SPEC.md algorithms in idiomatic Rustdesigns adn document the design in a design document. Use proper error handling, async where needed, marine vessel domain types. Generate complete working modules with tests.",
            "default_file": "docs/design/DESIGN_DOCUMENT.md"
        },
        "qa_analyst": {
            "system_prompt": "You are the QA Analyst for Rust GNC code. Read ARCH_SPEC.md and Rust src/gnc.rs. Write comprehensive cargo test cases, edge cases, marine failure modes. Output docs/qa/TEST_PLAN.md.",
            "default_file": "docs/qa/TEST_PLAN.md"
        },
        "tech_writer": {
            "system_prompt": "You are the Rust GNC Technical Writer using mistral:7b. Write clear Rust API docs, session summaries, GNC explanations. Structure with headers. Make marine math accessible.",
            "default_file": "docs/logs/session_log.md"
        }
    }
    return configs.get(role, configs["architect"])  # default to architect

def talk(role, user_prompt, target_file=None, mode="append"):
    # 1. Get agent behavior
    config = get_agent_config(role)
    
    # 2. Determine target file
    if not target_file:
        target_file = config["default_file"]
    target_path = Path("/app") / target_file
    
    # 3. Read relevant context files
    context = ""
    if role in ["qa", "tech_writer"]:
        spec_path = Path("/app") / "docs/spec/ARCH_SPEC.md"
        if spec_path.exists():
            with open(spec_path, "r") as f:
                context += f"\n\nCURRENT SPEC:\n" + f.read()

    
    # 4. Build full prompt
    full_prompt = f"{config['system_prompt']}\n\n{context}\n\nTASK: {user_prompt}\n\n"
    if mode == "append":
        full_prompt += "APPEND your response to the existing file."
    else:
        full_prompt += "REPLACE or create the file with your response."
    
    # 5. Call Ollama
    model = os.getenv("MODEL_NAME", "llama3.1:8b")
    url = "http://host.docker.internal:11434/api/generate"
    payload = {
        "model": model,
        "prompt": full_prompt,
        "stream": False
    }
    
    try:
        response = requests.post(url, json=payload)
        response.raise_for_status()
        result = response.json().get('response', '').strip()
        
        # 6. Write result
        target_path.parent.mkdir(parents=True, exist_ok=True)
        mode_flag = "a" if mode == "append" else "w"
        with open(target_path, mode_flag) as f:
            if mode == "append":
                f.write(f"\n\n---\n{result}\n---\n")
            else:
                f.write(result)
        
        print(f"✓ {role} → {target_file} ({mode})")
        return True
        
    except Exception as e:
        print(f"✗ FAILED: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python agent_talk.py <role> <prompt> [--file FILE] [--overwrite]")
        sys.exit(1)
    
    role = sys.argv[1]
    prompt = sys.argv[2]
    target_file = None
    mode = "append"
    
    for arg in sys.argv[3:]:
        if arg == "--overwrite":
            mode = "overwrite"
        elif arg.startswith("--file="):
            target_file = arg.split("=", 1)[1]
        elif arg.startswith("--file "):
            target_file = arg.split(" ", 1)[1]
    
    talk(role, prompt, target_file, mode)
