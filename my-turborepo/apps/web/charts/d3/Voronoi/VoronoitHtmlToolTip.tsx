import * as d3 from "d3";
import { Delaunay } from "d3-delaunay";
import { motion } from "framer-motion";
import { useState } from "react";

interface Datum {
  category: string;
  x: number;
  y: number;
  id: number;
}

const data: Datum[] = [
  { category: "cold", x: 14.2, y: 215, id: 1 },
  { category: "cold", x: 16.4, y: 325, id: 2 },
  { category: "cold", x: 11.9, y: 185, id: 3 },
  { category: "cold", x: 15.2, y: 332, id: 4 },
  { category: "cold", x: 18.5, y: 406, id: 5 },
  { category: "cold", x: 22.1, y: 522, id: 6 },
  { category: "cold", x: 19.4, y: 412, id: 7 },
  { category: "cold", x: 25.1, y: 614, id: 8 },
  { category: "cold", x: 23.4, y: 544, id: 9 },
  { category: "cold", x: 18.1, y: 421, id: 10 },
  { category: "cold", x: 22.6, y: 445, id: 11 },
  { category: "cold", x: 17.2, y: 408, id: 12 },
];

const height = 700;
const width = 700;

const margin = {
  left: 70,
  right: 70,
  top: 70,
  bottom: 70,
};
const VoronoiHoverTracker = () => {
  const [hovered, setHovered] = useState<Datum | null>(null);
  const yScale = d3
    .scaleLinear()
    .domain(d3.extent(data, (d) => d.y) as [number, number])
    .range([height, 0]);

  const xScale = d3
    .scaleLinear()
    .domain(d3.extent(data, (d) => d.x) as [number, number])
    .range([0, width]);

  // const points = data.map(({ x, y }) => [xScale(x), yScale(y)]);
  // const delaunay = Delaunay.from(points)
  const delaunay = Delaunay.from(
    data,
    (d) => xScale(d.x),
    (d) => yScale(d.y)
  );
  const voronoi = delaunay.voronoi([0, 0, width, height]);

  console.table(hovered);
  const chartWidth = width + margin.left + margin.right;
  const chartHeight = height + margin.top + margin.bottom;

  return (
    <div style={{ position: "relative" }}>
      <svg
        width={chartWidth}
        height={chartHeight}
        style={{ border: "0.1px solid pink" }}
        pointerEvents="none"
      >
        <g transform={`translate(${margin.left},${margin.top})`}>
          {data.map((point, i) => {
            const { x, y, id } = point;
            return (
              <g key={id}>
                <g transform={`translate(${xScale(x)},${yScale(y)})`}>
                  <text fontWeight="100" stroke="#bbb" fontSize="12">
                    {id}
                  </text>
                  <circle
                    r={3}
                    strokeWidth={3}
                    fill="pink"
                    stroke={hovered === point ? "green" : "red"}
                  />
                </g>

                <path
                  opacity={0.5}
                  fill="none"
                  stroke="teal"
                  pointerEvents="all"
                  d={voronoi.renderCell(i)}
                  onMouseEnter={() => setHovered(point)}
                  onMouseLeave={() => setHovered(null)}
                />
              </g>
            );
          })}
        </g>
      </svg>
      <motion.div
        animate={{
          x: hovered ? xScale(hovered.x) : 0,
          y: hovered ? yScale(hovered.y) : chartHeight / 2,
        }}
        style={{ background: "#eaeaea", position: "absolute", top: 0, left: 0 }}
      >
        {hovered?.x}:{hovered?.y}:{hovered?.id}lk
      </motion.div>
    </div>
  );
};

export default VoronoiHoverTracker;
{
  /* 
  <path
  pointerEvents="none"
  d={voronoi.render()}
  stroke="#eee"
  strokeWidth="2"
  fill="none"
/>
<path
  pointerEvents="none"
  d={voronoi.renderBounds()}
  stroke="#eaeaea"
  strokeWidth="2"
  fill="none"
/> 

*/
}

/* {data.map(({ x, y, category }, i) => (
  <polygon
    points={voronoi
      .cellPolygon(i)
      .map(([x, y]) => `${x}, ${y}`)
      .join(" ")}
    stroke="#fff"
    strokeWidth="2"
    fill="none"
    onMouseEnter={() => {
      setHoveredDatum({ x, y, category });
    }}
    onMouseLeave={() => {
      setHoveredDatum((currentDatum) =>
        currentDatum === {x,y,category} ? null : currentDatum
      );
    }}
    pointerEvents="visibleStroke"
    onClick={() => console.log("erer")}
  />
))} */
