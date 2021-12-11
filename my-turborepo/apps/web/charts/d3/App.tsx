import { css } from "@emotion/core";
import * as d3 from "d3";
import React, { StrictMode, useEffect, useState } from "react";

import { animated, useSpring } from "@react-spring/web";
import LineChart from "./LineChart/LineChart";
import VoronoiHoverTracker from "./Voronoi/Voronoi";
import styled from "@emotion/styled";

const Divi = styled.div({
  background: "green",
  color: "white",
  padding: 10,
});

const MyD3Charts = (): JSX.Element => {
  return (
    <StrictMode>
      <div
        style={{
          display: "grid",
          justifyItems: "center",
          alignItems: "center",
        }}
      >
        <p
          css={css({
            color: "blue",
          })}
        >
          this
        </p>
        <Divi>Tool tip hovering with Voronoi Polygon</Divi>
        <VoronoiHoverTracker />
        <SpringPlay />
        {/*         <Somethings />
          <Gesture />
          <Spring />
          <Chart /> */}
        <br />
        <br />
        <br />
        <br />
        <br />
        <LineChart />
      </div>
    </StrictMode>
  );
};

export default MyD3Charts;

export const SpringPlay = () => {
  const p = d3.line()([
    [10, 60],
    [40, 90],
    [60, 10],
    [190, 10],
  ]);
  const props = useSpring({
    testNumber: 1,
    from: { testNumber: 0 },
    config: { mass: 10, tension: 50, friction: 50, clamp: true },
  });
  console.log("fef", p, props);
  const { someX } = useSpring({
    someX: 400,
    from: { someX: 0 },
    // config: { mass: 10, tension: 50, friction: 50, },
  });
  return (
    /*  <animated.div>
      {props.testNumber.interpolate((val) => val.toFixed(2))}

     
    </animated.div> */
    <animated.svg stroke="red" fill="none">
      <path d={String(p)} strokeDashoffset={someX.to(val => val.toFixed(0)).get()} strokeDasharray={400} />
    </animated.svg>
  );
};