// import * as d3 from "d3-shape";
import * as d3 from "d3";
// import { extent } from "d3-array";
import { axisBottom, axisLeft } from "d3-axis";
import { scaleLinear, scaleTime } from "d3-scale";
import React, { FC } from "react";
import { animated, useSpring } from "@react-spring/web";

const { extent, line } = d3;
interface Datum {
  date: Date;
  value: number;
}

const margin = {
  left: 55,
  right: 95,
  bottom: 15,
  top: 55,
};

const padding = {
  left: 5,
  right: 5,
  bottom: 5,
  top: 5,
};

const width = 600;
const height = 300;
const chartWidth = width - margin.left - margin.right;
const chartHeight = height - margin.top - margin.bottom;

const Chart: FC = ({ children }) => {
  const data = [
    { date: new Date(2007, 3, 24), value: 93.24 },
    { date: new Date(2007, 3, 25), value: 95.35 },
    { date: new Date(2007, 3, 26), value: 98.84 },
    { date: new Date(2007, 3, 27), value: 99.92 },
    { date: new Date(2007, 3, 30), value: 99.8 },
    { date: new Date(2007, 4, 1), value: 99.47 },
  ];

  const props2 = useSpring({
    x: 0,
    from: { x: 1000 },
    config: { friction: 50, mass: 1, tension: 102 },
  });

  const extentY = extent(data, (d) => d.value) as [number, number];
  const extentX = extent(data, (d) => d.date) as [Date, Date];

  const xScale = scaleTime().domain(extentX).range([0, chartWidth]).nice();
  const yScale = scaleLinear().domain(extentY).range([0, chartHeight]).nice();

  const xAxis = axisBottom(xScale);
  const yAxis = axisLeft(yScale);

  const generateLine = line<Datum>()
    .curve(d3.curveCardinal)
    .x((d) => xScale(d.date))
    .y((d) => yScale(d.value));

  return (
    <animated.div>
      {" "}
      <animated.svg
        style={{ background: "#eaeaea" }}
        width={width}
        height={height + 100}
        fill="none"
        stroke="green"
      >
        <animated.g transform={`translate(${margin.left}, ${margin.top})`}>
          <rect
            x={0}
            y={0}
            height={chartHeight}
            width={chartWidth}
            opacity={0.6}
            stroke="black"
          />
          <animated.path
            d={generateLine(data)!}
            strokeDashoffset={props2.x}
            strokeDasharray="1000"
          />
          {data.map((d) => (
            <circle
              key={d.value}
              cy={yScale(d.value)}
              cx={xScale(d.date)}
              r="3"
              fill="black"
            />
          ))}
          <g transform={`translate(${chartWidth + 4},0)`}>
            {data.map((d) => (
              <text key={d.value} x={0} y={yScale(d.value)}>
                {d.value}
              </text>
            ))}
          </g>

          {/* X-axis */}
          <g transform={`translate(0, ${chartHeight + 13}) rotate(90)`}>
            <text x={xScale(d3.min(data, (d) => d.date)!)} y={0} dx="0em">
              {d3.min(data, (d) => d.date)?.toLocaleDateString()}
            </text>
            <text x={xScale(d3.max(data, (d) => d.date)!)} y={0}>
              {d3.max(data, (d) => d.date)?.toLocaleDateString()}
            </text>
          </g>
        </animated.g>
      </animated.svg>
    </animated.div>
  );
};

export default Chart;
