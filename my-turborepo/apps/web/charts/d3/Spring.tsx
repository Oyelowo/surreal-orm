import React from "react";
import { animated, interpolate, useSpring } from "react-spring";
const Spring = () => {
  const { o, xyz, color, a, b, c } = useSpring({
    from: { o: 0, xyz: [0, 0, 0], a: 0, b: 0, c: 0, color: "red" },
    o: 1,

    a: 10,
    b: 20,
    c: 5,

    xyz: [10, 20, 5],
    color: "green",
    config: {
      friction: 20,
    },
  });

  const [x, y, z] = xyz.get();
  console.log(x, y, z);
  xyz.interpolate((o) => console.log(o));
  return (
    <animated.div
      style={{
        // If you can, use plain animated values like always, ...
        // You would do that in all cases where values "just fit"
        color,
        // Unless you need to interpolate them
        background: o.interpolate((o) => `rgba(210, 57, 77, ${o})`),
        // Which works with arrays as well
        transform: interpolate(
          [a, b, c],
          (a, b, c) => `translate3d(${a}px, ${b}px, ${c}px)`
        ),
        // If you want to combine multiple values use the "interpolate" helper
        border: interpolate(
          [o, color as any],
          (o, c) => `${o * 10}px solid ${c}`
        ),
        // You can also form ranges, even chain multiple interpolations
        padding: o
          .interpolate({ range: [0, 0.5, 1], output: [0, 40, 10] })
          .interpolate((o) => `${o}%`),
        // Interpolating strings (like up-front) through ranges is allowed ...
        borderColor: o.interpolate({
          range: [0, 0.25, 0.5, 0.75, 1],
          output: ["red", "yellow", "#ffaabb", "cyan", "purple"],
        }),
        // There's also a shortcut for plain, optionless ranges ...
        opacity: o.interpolate({
          range: [0.1, 0.2, 0.6, 1],
          output: [1, 0.1, 0.5, 1],
        }),
      }}
    >
      {o.interpolate((n) => n.toFixed(2)) /* innerText interpolation ... */}
    </animated.div>
  );
};

export default Spring;
