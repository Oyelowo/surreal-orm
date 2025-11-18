import sys
from pathlib import Path

# Add the package root so tests can import `surreal_orm` directly
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
