import { useState, useRef, useEffect, useCallback } from "react";

// ─── The actual atom graph data from tonight's audit ───
const NODES = [
  // GROUND LAYER
  { id: "GOD", label: "God (Axiom)", type: "axiom", x: 500, y: 30, grade: "—", desc: "The one axiom. Self-grounding ground." },
  
  // GOD PROPERTIES
  { id: "G01", label: "Self-Grounding", type: "claim", x: 150, y: 120, grade: "C", desc: "G must ground itself" },
  { id: "G02", label: "Omniscience", type: "claim", x: 300, y: 120, grade: "C", desc: "G knows all that is knowable" },
  { id: "G03", label: "Omnipotence", type: "claim", x: 450, y: 120, grade: "C", desc: "G can actualize any consistent state" },
  { id: "G06", label: "Perfection", type: "claim", x: 600, y: 120, grade: "C", desc: "Maximal coherence = perfection" },
  { id: "G07", label: "Moral Nature", type: "claim", x: 750, y: 120, grade: "C", desc: "G is necessarily moral" },
  { id: "G08", label: "Divine Negation", type: "claim", x: 900, y: 120, grade: "C", desc: "What G cannot do" },

  // FOUNDATIONS
  { id: "F01", label: "Existence", type: "claim", x: 100, y: 230, grade: "B", desc: "Why something rather than nothing" },
  { id: "F02", label: "Information", type: "claim", x: 250, y: 230, grade: "C", desc: "Information is fundamental" },
  { id: "F03", label: "Coherence", type: "claim", x: 400, y: 230, grade: "C", desc: "The coherence imperative" },
  { id: "F04", label: "Actuality", type: "claim", x: 550, y: 230, grade: "C", desc: "Potentiality → actualization" },
  { id: "F05", label: "Agency", type: "claim", x: 700, y: 230, grade: "C", desc: "Agency is real" },

  // MASTER EQUATION
  { id: "ME", label: "Master Equation\nχ = C_W[∏ Xᵢ]", type: "equation", x: 400, y: 350, grade: "B", desc: "The full product form" },
  { id: "CHI", label: "χ Properties", type: "claim", x: 600, y: 350, grade: "—", desc: "Bounded, product, veto, dimensionless" },

  // NINE VARIABLES
  { id: "G_var", label: "G\nNegentropy", type: "variable", x: 50, y: 470, grade: "B", desc: "External negentropy influx" },
  { id: "M_var", label: "M\nAlignment", type: "variable", x: 150, y: 470, grade: "B", desc: "Alignment cosine" },
  { id: "E_var", label: "E\nFidelity", type: "variable", x: 250, y: 470, grade: "B", desc: "Signal propagation fidelity" },
  { id: "S_var", label: "S_eff\nEntropy", type: "variable", x: 350, y: 470, grade: "B", desc: "Effective entropy factor" },
  { id: "T_var", label: "T\nTime", type: "variable", x: 450, y: 470, grade: "B", desc: "Temporal integration" },
  { id: "K_var", label: "K\nCompression", type: "variable", x: 550, y: 470, grade: "B", desc: "Information compression ratio" },
  { id: "R_var", label: "R\nPhase", type: "variable", x: 650, y: 470, grade: "B", desc: "Phase transition indicator" },
  { id: "Q_var", label: "Q\nSuperposition", type: "variable", x: 750, y: 470, grade: "B", desc: "Unresolved possibility" },
  { id: "F_var", label: "F\nCorrelation", type: "variable", x: 850, y: 470, grade: "B", desc: "Non-local correlation" },
  { id: "C_var", label: "C\nIntegration", type: "variable", x: 950, y: 470, grade: "B", desc: "Total integration measure" },

  // THEOREMS & PROOFS
  { id: "VETO", label: "Zero Collapse\n(Lean ✓)", type: "proof", x: 300, y: 580, grade: "A", desc: "Any factor = 0 → χ = 0. Lean-verified." },
  { id: "GRACE_CHAIN", label: "Grace Chain\n(Lean ✓)", type: "proof", x: 500, y: 580, grade: "A", desc: "Grace → Faith → Hope → Salvation. Only A in corpus." },
  { id: "LAGRANGIAN", label: "Lowe Lagrangian\n(Python ✓)", type: "proof", x: 700, y: 580, grade: "B", desc: "Mass matrix + Hamiltonian verified" },
  { id: "FORMAL_SPEC", label: "Formal Spec", type: "proof", x: 150, y: 580, grade: "B", desc: "Lean guardrail supported" },

  // HUMAN ONTOLOGY
  { id: "H01", label: "Constitution", type: "claim", x: 50, y: 690, grade: "C", desc: "Body, Soul, Spirit" },
  { id: "H03", label: "Soul Field", type: "claim", x: 200, y: 690, grade: "C", desc: "S as enduring bearer" },
  { id: "H06", label: "Consciousness\n& Φ", type: "claim", x: 350, y: 690, grade: "C", desc: "Consciousness and integrated information" },

  // FRUITS & SPIRITUAL TERMS
  { id: "FRUITS", label: "Fruits of Spirit\n(9 terms)", type: "claim", x: 550, y: 690, grade: "C", desc: "Lean conditional (has sorry)" },
  { id: "GRACE_OP", label: "Grace Operator\n(Lean sorry)", type: "claim", x: 750, y: 690, grade: "C", desc: "grace_injective incomplete" },

  // TEN LAWS
  { id: "L1", label: "Law 1\nNewton-Grace", type: "law", x: 50, y: 800, grade: "B", desc: "Near-isomorphic" },
  { id: "L4", label: "Law 4\nYukawa-Agape", type: "law", x: 200, y: 800, grade: "B", desc: "Derived (confinement)" },
  { id: "L5", label: "Law 5\nClausius-Judgment", type: "law", x: 350, y: 800, grade: "B", desc: "Derived (Cross uniqueness)" },
  { id: "L6", label: "Law 6\nShannon-Logos", type: "law", x: 500, y: 800, grade: "B", desc: "Split (Shannon + Kolmogorov)" },
  { id: "L9", label: "Law 9\nFermi-Conservation", type: "law", x: 650, y: 800, grade: "B", desc: "Derived (Noether + CP)" },
  { id: "L10", label: "Law 10\nCoherence-Christ", type: "law", x: 800, y: 800, grade: "B", desc: "Definitional (wrapper)" },

  // PHYSICS MIRRORS (ISO candidates)
  { id: "ISO_LANG", label: "Langevin\nDynamics", type: "mirror", x: 200, y: 910, grade: "L6", desc: "Maps AND breaks. Grace ≠ noise." },
  { id: "ISO_BOLTZ", label: "Boltzmann\nFactor", type: "mirror", x: 350, y: 910, grade: "L6", desc: "S_eff = e^(-αS). Same form." },
  { id: "ISO_ISING", label: "Ising Model\nPhase Transition", type: "mirror", x: 500, y: 910, grade: "L6", desc: "Fruits critical behavior" },
  { id: "ISO_SHANNON", label: "Shannon\nCapacity", type: "mirror", x: 650, y: 910, grade: "L7", desc: "E IS this. Definitional." },
  { id: "ISO_AND", label: "AND Gate\nBoolean", type: "mirror", x: 800, y: 910, grade: "L7", desc: "Veto IS AND. Lean proved." },
];

const EDGES = [
  // God → properties
  { from: "GOD", to: "G01", type: "generates" },
  { from: "GOD", to: "G02", type: "generates" },
  { from: "GOD", to: "G03", type: "generates" },
  { from: "GOD", to: "G06", type: "generates" },
  { from: "GOD", to: "G07", type: "generates" },
  { from: "GOD", to: "G08", type: "generates" },
  // God → foundations
  { from: "GOD", to: "F01", type: "generates" },
  { from: "F01", to: "F02", type: "depends" },
  { from: "F02", to: "F03", type: "depends" },
  { from: "F03", to: "F04", type: "depends" },
  { from: "F04", to: "F05", type: "depends" },
  // Foundations → ME
  { from: "F02", to: "ME", type: "grounds" },
  { from: "F03", to: "ME", type: "grounds" },
  { from: "G06", to: "ME", type: "grounds" },
  { from: "ME", to: "CHI", type: "defines" },
  // ME → variables
  { from: "ME", to: "G_var", type: "contains" },
  { from: "ME", to: "M_var", type: "contains" },
  { from: "ME", to: "E_var", type: "contains" },
  { from: "ME", to: "S_var", type: "contains" },
  { from: "ME", to: "T_var", type: "contains" },
  { from: "ME", to: "K_var", type: "contains" },
  { from: "ME", to: "R_var", type: "contains" },
  { from: "ME", to: "Q_var", type: "contains" },
  { from: "ME", to: "F_var", type: "contains" },
  { from: "ME", to: "C_var", type: "contains" },
  // Variables → proofs
  { from: "CHI", to: "VETO", type: "proves" },
  { from: "G_var", to: "GRACE_CHAIN", type: "proves" },
  { from: "ME", to: "LAGRANGIAN", type: "proves" },
  { from: "ME", to: "FORMAL_SPEC", type: "proves" },
  // Variables → human ontology
  { from: "C_var", to: "H06", type: "grounds" },
  { from: "F05", to: "H01", type: "grounds" },
  { from: "H01", to: "H03", type: "depends" },
  { from: "H01", to: "H06", type: "depends" },
  // Variables → fruits
  { from: "ME", to: "FRUITS", type: "derives" },
  { from: "G_var", to: "GRACE_OP", type: "derives" },
  { from: "FRUITS", to: "GRACE_OP", type: "depends" },
  // ME → laws
  { from: "ME", to: "L1", type: "derives" },
  { from: "ME", to: "L4", type: "derives" },
  { from: "ME", to: "L5", type: "derives" },
  { from: "ME", to: "L6", type: "derives" },
  { from: "ME", to: "L9", type: "derives" },
  { from: "ME", to: "L10", type: "derives" },
  // Laws → physics mirrors
  { from: "CHI", to: "ISO_LANG", type: "mirror" },
  { from: "S_var", to: "ISO_BOLTZ", type: "mirror" },
  { from: "FRUITS", to: "ISO_ISING", type: "mirror" },
  { from: "E_var", to: "ISO_SHANNON", type: "mirror" },
  { from: "VETO", to: "ISO_AND", type: "mirror" },
];

const TYPE_COLORS = {
  axiom: "#e8a838",
  claim: "#6d9cff",
  equation: "#c084fc",
  variable: "#34d399",
  proof: "#22c55e",
  law: "#f97316",
  mirror: "#ec4899",
};

const GRADE_BORDER = {
  A: "#22c55e",
  B: "#6d9cff",
  C: "#f59e0b",
  D: "#ef4444",
};

const EDGE_COLORS = {
  generates: "#e8a83860",
  depends: "#6d9cff40",
  grounds: "#c084fc40",
  contains: "#34d39940",
  proves: "#22c55e80",
  derives: "#f9731640",
  defines: "#c084fc60",
  mirror: "#ec489960",
};

export default function AtomGraph() {
  const canvasRef = useRef(null);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(0.85);
  const [dragging, setDragging] = useState(null);
  const [selected, setSelected] = useState(null);
  const [hoveredNode, setHoveredNode] = useState(null);
  const [nodes, setNodes] = useState(NODES);
  const lastMouse = useRef({ x: 0, y: 0 });

  const toScreen = useCallback((x, y) => ({
    sx: x * zoom + pan.x,
    sy: y * zoom + pan.y,
  }), [zoom, pan]);

  const toWorld = useCallback((sx, sy) => ({
    x: (sx - pan.x) / zoom,
    y: (sy - pan.y) / zoom,
  }), [zoom, pan]);

  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const W = canvas.width = canvas.offsetWidth * 2;
    const H = canvas.height = canvas.offsetHeight * 2;
    ctx.scale(2, 2);
    ctx.clearRect(0, 0, W, H);

    // Background
    ctx.fillStyle = "#0f1015";
    ctx.fillRect(0, 0, W / 2, H / 2);

    // Grid
    ctx.strokeStyle = "#1a1b25";
    ctx.lineWidth = 0.5;
    const gridSize = 50 * zoom;
    const offX = pan.x % gridSize;
    const offY = pan.y % gridSize;
    for (let x = offX; x < W / 2; x += gridSize) {
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, H / 2); ctx.stroke();
    }
    for (let y = offY; y < H / 2; y += gridSize) {
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(W / 2, y); ctx.stroke();
    }

    // Edges
    for (const edge of EDGES) {
      const fromNode = nodes.find(n => n.id === edge.from);
      const toNode = nodes.find(n => n.id === edge.to);
      if (!fromNode || !toNode) continue;

      const from = toScreen(fromNode.x, fromNode.y);
      const to = toScreen(toNode.x, toNode.y);

      const isHighlighted = selected && (edge.from === selected || edge.to === selected);

      ctx.strokeStyle = isHighlighted
        ? (EDGE_COLORS[edge.type] || "#ffffff40").replace(/[0-9a-f]{2}$/, "ff")
        : (EDGE_COLORS[edge.type] || "#ffffff20");
      ctx.lineWidth = isHighlighted ? 2 : 1;
      ctx.setLineDash(edge.type === "mirror" ? [4, 4] : []);

      ctx.beginPath();
      ctx.moveTo(from.sx, from.sy);
      ctx.lineTo(to.sx, to.sy);
      ctx.stroke();

      // Arrow
      const angle = Math.atan2(to.sy - from.sy, to.sx - from.sx);
      const headLen = 8;
      const mx = to.sx - Math.cos(angle) * 20 * zoom;
      const my = to.sy - Math.sin(angle) * 20 * zoom;
      ctx.beginPath();
      ctx.moveTo(mx, my);
      ctx.lineTo(mx - headLen * Math.cos(angle - 0.4), my - headLen * Math.sin(angle - 0.4));
      ctx.moveTo(mx, my);
      ctx.lineTo(mx - headLen * Math.cos(angle + 0.4), my - headLen * Math.sin(angle + 0.4));
      ctx.stroke();
    }
    ctx.setLineDash([]);

    // Nodes
    for (const node of nodes) {
      const { sx, sy } = toScreen(node.x, node.y);
      const isSelected = selected === node.id;
      const isHovered = hoveredNode === node.id;
      const w = node.type === "equation" ? 80 : node.type === "axiom" ? 70 : 55;
      const h = node.label.includes("\n") ? 36 : 26;

      // Glow
      if (isSelected || isHovered) {
        ctx.shadowColor = TYPE_COLORS[node.type];
        ctx.shadowBlur = 12;
      }

      // Box
      ctx.fillStyle = isSelected ? TYPE_COLORS[node.type] + "30" : "#181a22";
      ctx.strokeStyle = GRADE_BORDER[node.grade] || TYPE_COLORS[node.type] || "#333";
      ctx.lineWidth = isSelected ? 2 : 1;
      ctx.beginPath();
      ctx.roundRect(sx - w, sy - h, w * 2, h * 2, 6);
      ctx.fill();
      ctx.stroke();
      ctx.shadowBlur = 0;

      // Grade badge
      if (node.grade && node.grade !== "—") {
        const badgeColor = GRADE_BORDER[node.grade] || "#6b7280";
        ctx.fillStyle = badgeColor;
        ctx.font = "bold 8px Inter, sans-serif";
        ctx.textAlign = "right";
        ctx.fillText(node.grade, sx + w - 4, sy - h + 12);
      }

      // Label
      ctx.fillStyle = isSelected ? "#ffffff" : "#c8ccd4";
      ctx.font = `${isSelected ? "bold " : ""}10px Inter, sans-serif`;
      ctx.textAlign = "center";
      const lines = node.label.split("\n");
      lines.forEach((line, i) => {
        ctx.fillText(line, sx, sy - 2 + i * 13);
      });

      // Type indicator
      ctx.fillStyle = TYPE_COLORS[node.type] + "80";
      ctx.font = "7px Inter, sans-serif";
      ctx.fillText(node.type.toUpperCase(), sx, sy + h - 4);
    }
  }, [nodes, pan, zoom, selected, hoveredNode, toScreen]);

  useEffect(() => { draw(); }, [draw]);

  const findNode = useCallback((sx, sy) => {
    const { x, y } = toWorld(sx, sy);
    for (let i = nodes.length - 1; i >= 0; i--) {
      const n = nodes[i];
      const w = n.type === "equation" ? 80 : n.type === "axiom" ? 70 : 55;
      const h = n.label.includes("\n") ? 36 : 26;
      if (x >= n.x - w && x <= n.x + w && y >= n.y - h && y <= n.y + h) return n;
    }
    return null;
  }, [nodes, toWorld]);

  const onMouseDown = (e) => {
    const rect = canvasRef.current.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;
    const node = findNode(sx, sy);
    if (node) {
      setDragging(node.id);
      setSelected(node.id);
    } else {
      setSelected(null);
      setDragging("pan");
    }
    lastMouse.current = { x: e.clientX, y: e.clientY };
  };

  const onMouseMove = (e) => {
    const rect = canvasRef.current.getBoundingClientRect();
    const sx = e.clientX - rect.left;
    const sy = e.clientY - rect.top;

    if (!dragging) {
      const node = findNode(sx, sy);
      setHoveredNode(node ? node.id : null);
      return;
    }

    const dx = e.clientX - lastMouse.current.x;
    const dy = e.clientY - lastMouse.current.y;
    lastMouse.current = { x: e.clientX, y: e.clientY };

    if (dragging === "pan") {
      setPan(p => ({ x: p.x + dx, y: p.y + dy }));
    } else {
      setNodes(prev => prev.map(n =>
        n.id === dragging ? { ...n, x: n.x + dx / zoom, y: n.y + dy / zoom } : n
      ));
    }
  };

  const onMouseUp = () => setDragging(null);

  const onWheel = (e) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.92 : 1.08;
    setZoom(z => Math.max(0.3, Math.min(3, z * delta)));
  };

  const selectedNode = nodes.find(n => n.id === selected);
  const connectedEdges = selected
    ? EDGES.filter(e => e.from === selected || e.to === selected)
    : [];

  return (
    <div style={{ width: "100%", height: "100vh", background: "#0f1015", display: "flex", fontFamily: "'Inter', sans-serif" }}>
      <canvas
        ref={canvasRef}
        style={{ flex: 1, cursor: dragging === "pan" ? "grabbing" : dragging ? "move" : "default" }}
        onMouseDown={onMouseDown}
        onMouseMove={onMouseMove}
        onMouseUp={onMouseUp}
        onMouseLeave={onMouseUp}
        onWheel={onWheel}
      />

      {/* Info panel */}
      <div style={{
        width: 280, background: "#181a22", borderLeft: "1px solid #1e2030",
        padding: 16, overflowY: "auto", flexShrink: 0,
      }}>
        <div style={{ fontSize: 14, fontWeight: 700, color: "#e8a838", marginBottom: 4 }}>
          Atom Dependency Map
        </div>
        <div style={{ fontSize: 10, color: "#4b5563", marginBottom: 16 }}>
          51 atoms · 47 edges · drag nodes · scroll to zoom
        </div>

        {/* Legend */}
        <div style={{ fontSize: 10, color: "#6b7280", marginBottom: 12 }}>
          {Object.entries(TYPE_COLORS).map(([type, color]) => (
            <div key={type} style={{ display: "flex", alignItems: "center", gap: 6, marginBottom: 3 }}>
              <span style={{ width: 8, height: 8, borderRadius: 2, background: color, display: "inline-block" }} />
              {type}
            </div>
          ))}
        </div>

        <div style={{ height: 1, background: "#1e2030", margin: "12px 0" }} />

        {selectedNode ? (
          <>
            <div style={{ fontSize: 13, fontWeight: 600, color: "#e2e8f0", marginBottom: 4 }}>
              {selectedNode.label.replace("\n", " ")}
            </div>
            <div style={{
              fontSize: 11, color: GRADE_BORDER[selectedNode.grade] || "#6b7280",
              fontWeight: 700, marginBottom: 8,
            }}>
              Grade: {selectedNode.grade} · {selectedNode.type}
            </div>
            <div style={{ fontSize: 12, color: "#94a3af", lineHeight: 1.5, marginBottom: 12 }}>
              {selectedNode.desc}
            </div>

            {connectedEdges.length > 0 && (
              <>
                <div style={{ fontSize: 10, color: "#6b7280", fontWeight: 600, marginBottom: 4 }}>
                  CONNECTIONS ({connectedEdges.length})
                </div>
                {connectedEdges.map((e, i) => {
                  const other = e.from === selected ? e.to : e.from;
                  const otherNode = nodes.find(n => n.id === other);
                  const dir = e.from === selected ? "→" : "←";
                  return (
                    <div key={i} style={{
                      fontSize: 11, color: "#9ca3af", padding: "3px 0",
                      cursor: "pointer",
                    }} onClick={() => setSelected(other)}>
                      {dir} <span style={{ color: TYPE_COLORS[otherNode?.type] }}>{otherNode?.label.replace("\n", " ")}</span>
                      <span style={{ color: "#4b5563", marginLeft: 6 }}>{e.type}</span>
                    </div>
                  );
                })}
              </>
            )}
          </>
        ) : (
          <div style={{ fontSize: 12, color: "#4b5563" }}>
            Click a node to see its details and connections.
          </div>
        )}
      </div>
    </div>
  );
}
