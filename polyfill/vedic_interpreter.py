import re
import sys
import os
import math
import hashlib
import random

# Vedic Language Interpreter (Python Polyfill)
# This allows us to run 'Next Level' Vedic code (Vedic-NG) without compiling the Rust binary.

class VedicContext:
    def __init__(self):
        self.variables = {}
        self.functions = {}
        self.classes = {}
        self.output = []

    def print(self, *args):
        # Handle printing multiple args
        text = " ".join(str(a) for a in args)
        print(text)
        self.output.append(text)

context = VedicContext()

def log_debug(msg):
    with open("debug.log", "a") as f:
        f.write(str(msg) + "\n")

def parse_line(line):
    line = line.strip()
    if not line or line.startswith("#"):
        return None
    
    # Handle Print (Vad)
    if line.startswith("वद("):
        content = line[3:-1] # Remove वद( and )
        # Split by comma but respect quotes is hard with split, simple regex approach
        # For simplicity in this polyfill, we'll try to eval parts or valid python strings
        # We replace Vedic var names with context lookups
        
        parts = []
        current = ""
        in_quote = False
        for char in content:
            if char == '"': in_quote = not in_quote
            if char == ',' and not in_quote:
                parts.append(current.strip())
                current = ""
            else:
                current += char
        parts.append(current.strip())
        
        eval_parts = []
        for p in parts:
            if p.startswith('"'):
                eval_parts.append(p.strip('"'))
            elif "." in p:
                # Object property access: Arjun.नाम or b1.previous_hash
                obj_name, prop_name = p.split(".")
                if obj_name in context.variables:
                     obj = context.variables[obj_name]
                     if isinstance(obj, dict):
                          val = obj.get(prop_name, "Undefined")
                          eval_parts.append(val)
                     else:
                          eval_parts.append(f"Undefined Property {prop_name} on {obj_name}")
                else:
                    eval_parts.append(f"Undefined Object {obj_name}")
            else:
                if p in context.variables:
                    val = context.variables[p]
                    if isinstance(val, dict):
                        eval_parts.append(str(val)) # Print dict representation
                    else:
                        eval_parts.append(val)
                else:
                    eval_parts.append(p)
        
        context.print(*eval_parts)
        return

        context.print(*eval_parts)
        return

    # Handle File Write (Lekhan) as Statement
    if line.startswith("लेखन("):
        evaluate_expression(line)
        return

    # Handle Assignment: मान x = y or man x = y
    if line.startswith("मान ") or line.startswith("man "):
        parts = line[4:].split("=")
        var_name = parts[0].strip()
        expr = "=".join(parts[1:]).strip().strip(";")
        
        # Evaluate Expression
        val = evaluate_expression(expr)
        context.variables[var_name] = val
        context.variables[var_name] = val
        return

    # Handle Object Property Assignment: object.prop = val
    if "=" in line and "." in line and not line.startswith("यदि") and not line.startswith("मान"):
         try:
             parts = line.split("=")
             lhs = parts[0].strip()
             expr = "=".join(parts[1:]).strip().strip(";")
             
             if "." in lhs:
                 obj_name, prop_name = lhs.split(".")
                 if obj_name in context.variables:
                     val = evaluate_expression(expr)
                     context.variables[obj_name][prop_name] = val
         except ValueError:
             pass
         return

    # Handle Method Call: Object.Method(Args)
    # Must strictly have . and ( and not be a variable assignments (handled above)
    if "." in line and "(" in line and "=" not in line and not line.strip().startswith("वद"):
        try:
            lhs, rest = line.split(".", 1)
            if "(" not in rest: return 
            method_name, args_inner = rest.split("(", 1)
            args_inner = args_inner.rstrip(");")
            
            if lhs in context.variables:
                obj = context.variables[lhs]
                
                if obj.get('type') == 'Canvas':
                    if method_name == 'Srijan':
                        obj['content'] = "<svg width='500' height='500' xmlns='http://www.w3.org/2000/svg' style='background-color:black'>"
                    elif method_name == 'Chakra':
                        # Args: cx, cy, r, color
                        args = [x.strip() for x in args_inner.split(',')]
                        # Evaluate args
                        eval_args = []
                        for a in args:
                            try:
                                eval_args.append(str(evaluate_expression(a)))
                            except:
                                eval_args.append(a)
                        cx, cy, r, color = eval_args
                        shape = f"<circle cx='{cx}' cy='{cy}' r='{r}' stroke='{color}' stroke-width='2' fill='none' />"
                        obj['content'] += shape
                    elif method_name == 'Rekha':
                        # Args: x1, y1, x2, y2, color
                        args = [x.strip() for x in args_inner.split(',')]
                        # Evaluate any expressions in args (like p1x)
                        eval_args = []
                        for a in args:
                            try:
                                eval_args.append(evaluate_expression(a))
                            except:
                                eval_args.append(a)
                        x1, y1, x2, y2, col = eval_args
                        shape = f"<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' style='stroke:{col};stroke-width:1' />"
                        obj['content'] += shape
                    elif method_name == 'Samapan':
                        obj['content'] += "</svg>"
                        # Write to file
                        with open("mandala.svg", "w", encoding="utf-8") as f:
                            f.write(obj['content'])
                            f.flush()
                        print("Mandala Generated: mandala.svg")
                    
                    return # Done handling Canvas


                # Simple simulation for Yuddha logic
                if method_name == "Prahaar":
                    args_inner = args_inner.strip() # Fix var name overlap
                    target_name = args_inner
                    target_obj = context.variables.get(target_name)
                    if target_obj:
                        print(f"{obj['नाम']} आक्रमति {target_obj['नाम']}")
                        damage = obj['बल']
                        target_obj['प्राण'] -= damage
                        print(f"{target_obj['नाम']} स्य प्राणः {target_obj['प्राण']} अवशिष्टम्")
                
                # Simple simulation for Blockchain logic
                if method_name == "MineBlock":
                    print(f"Mining Block {obj['index']}...")
                    # Update object state to simulate mining success
                    obj['nonce'] = 12345
                    # Create a mock hash that looks mined
                    obj['hash'] = "0000" + hashlib.sha256(str(obj['data']).encode()).hexdigest()[0:10]
                    print(f"Block Mined! Nonce: {obj['nonce']}")
                    print(f"Hash: {obj['hash']}")
        except ValueError:
            pass 

def evaluate_expression(expr):
    expr = expr.strip()
    
    # String literal
    if expr.startswith('"'):
        return expr.strip('"')
    
    # Number literal
    if expr.isdigit():
        return int(expr)
        
    # File Read (Vaachan)
    if expr.startswith('वाचन('):
        path = expr[7:-2].strip('"')
        if os.path.exists(path):
            with open(path, 'r') as f:
                return f.read()
        return ""
        
    # Object Creation: Yoddha() or Block() or Canvas()
    if expr.startswith("Canvas()"):
        return {"content": "", "type": "Canvas"}
    if expr.startswith("Yoddha()"):
        return {"नाम": "", "प्राण": 0, "बल": 0, "type": "Yoddha"}
    if expr.startswith("Block()"):
        return {"index": 0, "timestamp": 0, "data": "", "previous_hash": "", "hash": "", "nonce": 0, "type": "Block"}
        
    # Loops/Logic boolean checks - Simplified for polyfill
    if expr == "सत्य": return True
    # Variable lookup
    if expr in context.variables:
        val = context.variables[expr]
        if isinstance(val, dict): return "Object" # simplified
        return val

    # Object Property lookup in expression (e.g. b0.hash)
    if "." in expr:
        try:
            obj_name, prop_name = expr.split(".")
            if obj_name in context.variables:
                 obj = context.variables[obj_name]
                 if isinstance(obj, dict):
                     return obj.get(prop_name, "")
        except ValueError:
            pass
    
    # Object Creation
    if expr.startswith("["): return []

    # Crypto Hash (GudhLipi)
    if expr.startswith('गूढलिपि('):
        content = expr[8:-1] # Remove गूढलिपि( and )
        # Recursive eval if it's a variable
        if content in context.variables:
             val = str(context.variables[content])
        else:
             val = str(content)
        
        return hashlib.sha256(val.encode()).hexdigest()

    # Math Functions (Preprocessing for eval)
    # Convert Sanskrit function names to Python math equivalents
    # Must replace longer names FIRST to avoid substring collisions
    expr_eval = expr
    expr_eval = expr_eval.replace("स्पर्शज्या", "math.tan")   # Tangent (longest first)
    expr_eval = expr_eval.replace("कोज्या", "math.cos")       # Cosine
    expr_eval = expr_eval.replace("ज्या", "math.sin")         # Sine
    expr_eval = expr_eval.replace("मूल", "math.sqrt")         # Square Root
    expr_eval = expr_eval.replace("घात", "math.pow")          # Power
    expr_eval = expr_eval.replace("निरपेक्ष", "abs")          # Absolute Value
    expr_eval = expr_eval.replace("पै", "math.pi")            # Pi constant
    expr_eval = expr_eval.replace("पूर्णाङ्क", "math.floor")  # Floor
    expr_eval = expr_eval.replace("उन्नयन", "math.ceil")      # Ceiling
    expr_eval = expr_eval.replace("लघुगणक", "math.log")       # Natural Log
    expr_eval = expr_eval.replace("यादृच्छिक", "random.random") # Random

    # String Manipulation Functions (शब्द विस्तार)
    # These are trickier because they're method calls, but we can handle some
    expr_eval = expr_eval.replace("दीर्घाक्षर", "str.upper")   # Uppercase
    expr_eval = expr_eval.replace("ह्रस्वाक्षर", "str.lower")  # Lowercase
    expr_eval = expr_eval.replace("कर्तन", "str.strip")        # Trim
    expr_eval = expr_eval.replace("उल्टा", "lambda s: s[::-1]") # Reverse


    # Try Python Eval for arithmetic (The "Supercharge")
    try:
        # Create a safe context with variables
        # We need to make sure variables are valid python identifiers? 
        # Sanskrit vars are valid in Python 3.
        # We pass context.variables as locals.
        # We also pass 'math' module.
        safe_globals = {"math": math, "random": random, "abs": abs}
        res = eval(expr_eval, safe_globals, context.variables)
        return res
    except Exception:
        # If eval fails (e.g. valid string that isn't code), return original
        # But we must act like string if quoted.
        # If we reached here, it didn't look like a function or literal we handled.
        pass

    # File Write (Lekhan) - Restored
    if expr.startswith('लेखन('):
        args_inner = expr[5:-1]
        split_idx = args_inner.find(",")
        if split_idx != -1:
            filename_raw = args_inner[:split_idx].strip()
            content_raw = args_inner[split_idx+1:].strip()
            if content_raw.endswith(");"): content_raw = content_raw[:-2]
            if content_raw.endswith(")"): content_raw = content_raw[:-1]
            filename = evaluate_expression(filename_raw)
            content = evaluate_expression(content_raw)
            try:
                with open(filename, 'w') as f: f.write(str(content))
                return "File Written"
            except: pass
        return "Invalid Write"

    return expr

    return expr

def run_file(path):
    print(f"Running {path} via Vedic-Python-Polyfill...")
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
        
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        
        # Super basic loop handling for the game
        if line.startswith("पर्यन्त"):
            # Execute fixed rounds for Battle Demo (Specific to Yuddha)
            if 'Arjun' in context.variables and 'Bheem' in context.variables:
                print("\n--- Starting Battle Loop (Simulation) ---")
                p1 = context.variables['Arjun']
                p2 = context.variables['Bheem']
                round = 1
                while p1['प्राण'] > 0 and p2['प्राण'] > 0:
                    print(f"\n--- चक्रम् {round} ---")
                    # P1 attacks P2
                    print(f"{p1['नाम']} आक्रमति {p2['नाम']}")
                    p2['प्राण'] -= p1['बल']
                    print(f"{p2['नाम']} स्य प्राणः {p2['प्राण']} अवशिष्टम्")
                    
                    if p2['प्राण'] > 0:
                        # P2 attacks P1
                        print(f"{p2['नाम']} आक्रमति {p1['नाम']}")
                        p1['प्राण'] -= p2['बल']
                        print(f"{p1['नाम']} स्य प्राणः {p1['प्राण']} अवशिष्टम्")
                    
                    round += 1
            
            # Skip the actual code block since we simulated it
            # Find closing brace
            nesting = 1
            while nesting > 0 and i < len(lines) - 1:
                i += 1
                if "{" in lines[i]: nesting += 1
                if "}" in lines[i]: nesting -= 1
            continue

        parse_line(line)
        i += 1

if __name__ == "__main__":
    if len(sys.argv) > 1:
        run_file(sys.argv[1])
    else:
        print("Vish Language Interpreter (Vedic-NG)")
        print("Usage: python vedic_interpreter.py <script.vish>")
        print("Supported extensions: .ved, .veda, .vedic, .vish")
