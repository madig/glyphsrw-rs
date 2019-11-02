import json
from pathlib import Path

import openstep_plist


class ComplexEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, bytes):
            return obj.hex()
        # Let the base class default method raise the TypeError
        return json.JSONEncoder.default(self, obj)


for p in Path(".").glob("*.glyphs"):
    with open(p) as r:
        with open(p.with_suffix(".json"), "w+") as w:
            json.dump(openstep_plist.load(r), w, indent=1, cls=ComplexEncoder)
