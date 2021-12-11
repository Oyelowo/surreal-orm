import { HelloWorld } from "./HelloWorld";

import { render, screen } from "@testing-library/react"; // (or /dom, /vue, ...)

test("should show login form", () => {
  render(<HelloWorld />);
  const helloScreen = screen.getByText("Hello from the other side. I am Oyelowo");
  // Events and assertions...
});
