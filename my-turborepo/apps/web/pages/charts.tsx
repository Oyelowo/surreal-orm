
import { useAtom } from "jotai";
import React from "react";
import MyD3Charts from "../charts/d3/App";
import Somethings, { timeAtom } from "../charts/d3/jotai/Somethings";
import ReactEChartCustom from "../charts/echarts/ChartWithHooks";
import ReactEcharts from "../charts/echarts/ReactEcharts";

const Charts = () => {
  const [names, setStuff] = useAtom(timeAtom);
  return (
    <div>
      <Somethings />
      <br />
      <br />
      <br />
      <br />
      <br />
      <br />
      <br />

      <button onClick={() => setStuff(prev => [...prev, new Date().toISOString()])}>Add Time</button>
      <MyD3Charts />
    </div>
  );
};

export default Charts
