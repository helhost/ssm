# Part Asset Structure

This directory contains structured definitions for physical parts.
Each part is described using multiple layers with clearly separated responsibilities.

## Directory layout
```
assets
└── parts
    └── <system>
        └── <part_id>
            ├── part.toml
            ├── lattice.json
            ├── connectors.json
            └── visual
                ├── mesh.glb
                └── materials.json
```

## Layer responsibilities

### part.toml (metadata)
- Declares part identity and system
- Defines unit conventions
- References the files that define each layer
- Contains no geometry or occupancy data

### lattice.json (Layer 1: assembly lattice)
- Authoritative spatial footprint of the part
- Uses discrete units (studs horizontally, plates vertically)
- Defines occupied cells for collision and placement logic
- No visual or connector semantics

### connectors.json (Layer 3: semantics)
- Defines connection features (studs, tubes, pins, holes, etc.)
- Features are typed and positioned in local part coordinates
- Used for snapping, validation, and future constraint solving

### visual/* (Layer 2: geometry)
- Purely visual representation
- High-resolution meshes and materials
- Must align with lattice and connectors but does not define them
- Multiple visual representations may exist for a single part

## Coordinate conventions

- Right-handed coordinate system
- Origin at lower-left-bottom corner of lattice bounds
- +Z is up
- All connector and visual coordinates are expressed relative to the part origin
