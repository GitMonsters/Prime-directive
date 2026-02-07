import os

# Files we created/modified in this session
session_files = {
    'physics_world_model.py': 'Physics core system',
    'gaia_physics_integration.py': 'Integration layer',
    'test_physics_consciousness_integration.py': 'Integration tests',
    'formal_proof_verifier.py': 'Formal proofs',
    'ising_empathy_module.py': 'Empathy module (modified)',
    'gaia_consciousness_reasoning.py': 'Consciousness reasoning (modified)',
}

print("=" * 80)
print("MODEL SIZE ANALYSIS")
print("=" * 80)
print()

total_code_lines = 0
total_code_bytes = 0

print("CODE FILES:")
print("-" * 80)
for filename, description in session_files.items():
    path = filename
    if os.path.exists(path):
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            lines = len(f.readlines())
            total_code_lines += lines
        
        size_bytes = os.path.getsize(path)
        total_code_bytes += size_bytes
        
        size_kb = size_bytes / 1024
        print(f"{filename:45s} {lines:6d} lines  {size_kb:8.2f} KB  - {description}")

print("-" * 80)
print(f"{'TOTAL CODE':45s} {total_code_lines:6d} lines  {total_code_bytes/1024:8.2f} KB")
print()

# Documentation
doc_files = [
    'PHYSICS_WORLD_MODEL_GUIDE.md',
    'PHYSICS_WORLD_MODEL_SUMMARY.txt',
    'INTEGRATION_TEST_REPORT.md',
    'PHASE4_EMPATHY_IMPROVEMENT.md',
    'PHASE5_COMPLETION_REPORT.md',
    'SESSION_PHASE4_SUMMARY.txt',
    'SESSION_COMPLETE_SYSTEM_STATUS.md',
]

print("DOCUMENTATION FILES:")
print("-" * 80)
total_doc_lines = 0
total_doc_bytes = 0

for filename in doc_files:
    if os.path.exists(filename):
        with open(filename, 'r', encoding='utf-8', errors='ignore') as f:
            lines = len(f.readlines())
            total_doc_lines += lines
        
        size_bytes = os.path.getsize(filename)
        total_doc_bytes += size_bytes
        size_kb = size_bytes / 1024
        print(f"{filename:45s} {lines:6d} lines  {size_kb:8.2f} KB")

print("-" * 80)
print(f"{'TOTAL DOCUMENTATION':45s} {total_doc_lines:6d} lines  {total_doc_bytes/1024:8.2f} KB")
print()

print("SUMMARY:")
print("=" * 80)
print(f"Total Code:                {total_code_lines:6d} lines  {total_code_bytes/1024:8.2f} KB")
print(f"Total Documentation:       {total_doc_lines:6d} lines  {total_doc_bytes/1024:8.2f} KB")
print(f"Grand Total:               {total_code_lines + total_doc_lines:6d} lines  {(total_code_bytes + total_doc_bytes)/1024:8.2f} KB")
print()

print("MODEL PARAMETERS:")
print("-" * 80)
print("Physics Knowledge Base:")
print("  - Laws: 12")
print("  - Principles: 9")
print("  - Constants: 15+")
print("  - Domains: 5 physics + sacred geometry")
print()
print("GAIA System:")
print("  - Agents: 5 (configurable)")
print("  - Agent Size: 10-20 spins (configurable)")
print("  - Empathy Score Range: 0.0-1.0")
print("  - Confidence Range: 0.55-0.95")
print()

print("MEMORY FOOTPRINT (Runtime):")
print("-" * 80)
print("Physics Knowledge Base:    ~50 KB")
print("Router & Integration:      ~10 KB")
print("GAIA Agents (5×20):        ~500 KB (varies by agent size)")
print("Temporary Buffers:         ~100 KB")
print("-" * 80)
print("Total Runtime Memory:      ~660 KB (minimal)")
print()

print("DEPLOYMENT FOOTPRINT:")
print("-" * 80)
print("Code only (no docs):       {:.2f} MB".format(total_code_bytes / 1024 / 1024))
print("With documentation:        {:.2f} MB".format((total_code_bytes + total_doc_bytes) / 1024 / 1024))
print("With dependencies (torch): ~200 MB (torch itself)")
print()

print("=" * 80)
