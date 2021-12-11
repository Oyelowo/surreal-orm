import { CSSProperties, useEffect, useRef, useState } from "react";
// import { init, getInstanceByDom, ECharts, SetOptionOpts, EChartsOption } from "echarts";

// Tree-shakeable approach. //
import { init, use, ECharts, ComposeOption, SetOptionOpts, getInstanceByDom } from "echarts/core";
import {
  BarChart,
  BarSeriesOption,
  CandlestickChart,
  LinesChart,
  CandlestickSeriesOption,
  LineSeriesOption,
  PieChart,
  PieSeriesOption,
  LineChart,
} from "echarts/charts";

import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  DatasetComponent,
  TransformComponent,
  TitleComponentOption,
  GridComponentOption,
  TooltipComponentOption,
  DataZoomComponent,
  DataZoomSliderComponent,
  DataZoomInsideComponent,
  GraphicComponent,
  AxisPointerComponent,
  GridSimpleComponent,
  LegendComponent,
  LegendPlainComponent,
  LegendScrollComponent,
  SingleAxisComponent,
  MarkLineComponent,
  MarkPointComponent,
} from "echarts/components";

import { LabelLayout, UniversalTransition } from "echarts/features";
import { CanvasRenderer, SVGRenderer } from "echarts/renderers";

// Register the required components
const chartComponentsInUse = [
  TitleComponent,
  TooltipComponent,
  GridComponent,
  DatasetComponent,
  TransformComponent,
  BarChart,
  PieChart,
  LinesChart,
  CandlestickChart,
  GraphicComponent,
  GridSimpleComponent,
  DataZoomComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  SingleAxisComponent,
  AxisPointerComponent,
  LegendComponent,
  LegendPlainComponent,
  LegendScrollComponent,
  MarkLineComponent,
  MarkPointComponent,
  LineChart,
  CanvasRenderer,
  UniversalTransition,
  CanvasRenderer,
  // SVGRenderer
];

// Combine an Option type with only required components and charts via ComposeOption
export type ECOption = ComposeOption<
  | BarSeriesOption
  | PieSeriesOption
  | LineSeriesOption
  | TitleComponentOption
  | TooltipComponentOption
  | GridComponentOption
  | CandlestickSeriesOption
>;

export interface ReactEChartsProps {
  option: ECOption /* You may also just use the minimal type: EChartsOption */;
  style?: CSSProperties;
  settings?: SetOptionOpts;
  loading?: boolean;
  theme?: "light" | "dark";
}

export function useChart({ option, style, settings, loading, theme = "dark" }: ReactEChartsProps) {
  const chartRef = useRef<HTMLDivElement>(null);
  const [chart, setChart] = useState<ECharts>();

  useEffect(() => {
    use(chartComponentsInUse);
  }, []);

  useEffect(() => {
    // Initialize chart
    let chart: ECharts | undefined;
    if (chartRef.current !== null) {
      chart = init(chartRef.current, theme);

      // chart.on("legendselectchanged", function (params) {
      //   console.log(params);
      //   if (params.name === "lineData") {
      //     selectGraph(params);

      //     unselectGrap(params);
      //   }
      // });

      // function selectGraph(params) {
      //   chart.dispatchAction({
      //     type: "legendSelect",
      //     // legend name
      //     name: params.name,
      //   });
      // }

      // function unselectGrap(params) {
      //   for (const legend in params.selected) {
      //     if (legend !== params.name) {
      //       chart.dispatchAction({
      //         type: "legendUnSelect",
      //         // legend name
      //         name: legend,
      //       });
      //     }
      //   }
      // }
      setChart(chart);
    }

    // Add chart resize listener
    // ResizeObserver is leading to a bit janky UX
    function resizeChart() {
      chart?.resize();
    }
    window.addEventListener("resize", resizeChart);

    // Return cleanup function
    return () => {
      chart?.dispose();
      window.removeEventListener("resize", resizeChart);
    };
  }, [theme]);

  useEffect(() => {
    // Update chart
    if (chartRef.current !== null) {
      const chart = getInstanceByDom(chartRef.current);
      chart?.setOption(option, settings);
    }
  }, [option, settings, theme]); // Whenever theme changes we need to add option and setting due to it being deleted in cleanup function

  useEffect(() => {
    // Update chart
    if (chartRef.current !== null) {
      const chart = getInstanceByDom(chartRef.current);
      loading === true ? chart?.showLoading() : chart?.hideLoading();
    }
  }, [loading, theme]);

  return {
    ReactCharts: <div ref={chartRef} style={{ width: "100%", height: "100%", ...style }} />,
    chart,
  };
}

export function ReactEChartCustom({
  option,
  style,
  settings,
  loading,
  theme = "dark",
}: ReactEChartsProps): JSX.Element {
  const chartRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Initialize chart
    let chart: ECharts | undefined;
    if (chartRef.current !== null) {
      chart = init(chartRef.current, theme);
    }

    // Add chart resize listener
    // ResizeObserver is leading to a bit janky UX
    function resizeChart() {
      chart?.resize();
    }
    window.addEventListener("resize", resizeChart);

    // Return cleanup function
    return () => {
      chart?.dispose();
      window.removeEventListener("resize", resizeChart);
    };
  }, [theme]);

  useEffect(() => {
    // Update chart
    if (chartRef.current !== null) {
      const chart = getInstanceByDom(chartRef.current);
      chart?.setOption(option, settings);
    }
  }, [option, settings, theme]); // Whenever theme changes we need to add option and setting due to it being deleted in cleanup function

  useEffect(() => {
    // Update chart
    if (chartRef.current !== null) {
      const chart = getInstanceByDom(chartRef.current);
      loading === true ? chart?.showLoading() : chart?.hideLoading();
    }
  }, [loading, theme]);

  return <div ref={chartRef} style={{ width: "100%", height: 500, ...style }} />;
}

export default ReactEChartCustom;
